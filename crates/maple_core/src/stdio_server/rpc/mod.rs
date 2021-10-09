mod messages;
mod types;

use anyhow::{anyhow, Result};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;

pub use self::messages::method_call::MethodCall;
pub use self::messages::notification::Notification;
pub use self::types::{Call, Error, ErrorCode, Failure, Output, Params, RawMessage, Success};

#[derive(Serialize, Debug)]
pub struct RpcClient {
    /// Id of request to Vim created from the Rust side.
    #[serde(skip_serializing)]
    id: AtomicU64,
    /// Sender for sending message from Rust to Vim.
    #[serde(skip_serializing)]
    output_writer_tx: UnboundedSender<RawMessage>,
    /// Sender for passing the Vim response of request initiated from Rust.
    #[serde(skip_serializing)]
    output_reader_tx: UnboundedSender<(u64, oneshot::Sender<Output>)>,
}

impl RpcClient {
    /// Creates a new instance of [`RpcClient`].
    ///
    /// # Arguments
    ///
    /// * `reader`: a buffer reader on top of [`std::io::Stdin`].
    /// * `writer`: a buffer writer on top of [`std::io::Stdout`].
    pub fn new(
        reader: impl BufRead + Send + 'static,
        writer: impl Write + Send + 'static,
        sink: UnboundedSender<Call>,
    ) -> Self {
        // Channel for passing through the response from Vim and the request to Vim.
        let (output_reader_tx, output_reader_rx): (
            UnboundedSender<(u64, oneshot::Sender<Output>)>,
            _,
        ) = unbounded_channel();

        // A blocking task is necessary!
        tokio::task::spawn_blocking(move || {
            if let Err(error) = loop_read(reader, output_reader_rx, &sink) {
                tracing::error!(?error, "Thread stdio-reader exited");
            }
        });

        let (output_writer_tx, output_writer_rx) = unbounded_channel();
        // No blocking task.
        tokio::spawn(async move {
            if let Err(error) = loop_write(writer, output_writer_rx).await {
                tracing::error!(?error, "Thread stdio-writer exited");
            }
        });

        Self {
            id: Default::default(),
            output_reader_tx,
            output_writer_tx,
        }
    }

    /// Calls `call(method, params)` into Vim and return the result.
    pub async fn request<R: DeserializeOwned>(
        &self,
        method: impl AsRef<str>,
        params: impl Serialize,
    ) -> Result<R> {
        let id = self.id.fetch_add(1, Ordering::SeqCst);
        let method_call = MethodCall {
            id,
            method: method.as_ref().to_owned(),
            // call(method, args) where args expects a List in Vim, hence convert the params
            // to List unconditionally.
            params: to_array_or_none(params)?,
            session_id: 0u64, // Unused for now.
        };
        let (tx, rx) = oneshot::channel();
        self.output_reader_tx.send((id, tx))?;
        self.output_writer_tx
            .send(RawMessage::MethodCall(method_call))?;
        match rx.await? {
            Output::Success(ok) => Ok(serde_json::from_value(ok.result)?),
            Output::Failure(err) => Err(anyhow!("RpcClient request error: {:?}", err)),
        }
    }

    /// Sends a notification message to Vim.
    pub fn notify(&self, method: impl AsRef<str>, params: impl Serialize) -> Result<()> {
        let notification = Notification {
            method: method.as_ref().to_owned(),
            // call(method, args) where args expects a List in Vim, hence convert the params
            // to List unconditionally.
            params: to_array_or_none(params)?,
            session_id: None,
        };

        self.output_writer_tx
            .send(RawMessage::Notification(notification))?;

        Ok(())
    }

    /// Sends the response from Rust to Vim.
    pub fn output(&self, id: u64, output_result: Result<impl Serialize>) -> Result<()> {
        let output = match output_result {
            Ok(ok) => Output::Success(Success {
                id,
                result: serde_json::to_value(ok)?,
            }),
            Err(err) => Output::Failure(Failure {
                id,
                error: Error {
                    code: ErrorCode::InternalError,
                    message: err.to_string(),
                    data: None,
                },
            }),
        };

        self.output_writer_tx.send(RawMessage::Output(output))?;

        Ok(())
    }
}

/// Keep reading and processing the line from stdin.
fn loop_read(
    reader: impl BufRead,
    mut output_reader_rx: UnboundedReceiver<(u64, oneshot::Sender<Output>)>,
    sink: &UnboundedSender<Call>,
) -> Result<()> {
    let mut pending_outputs = HashMap::new();

    let mut reader = reader;
    loop {
        let mut line = String::new();
  