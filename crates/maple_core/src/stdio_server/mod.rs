mod handler;
mod input;
mod job;
mod provider;
mod rpc;
mod session;
mod state;
mod vim;

pub use self::input::InputHistory;
use self::input::{Event, ProviderEvent};
use self::provider::{create_provider, Context};
use self::rpc::{Call, MethodCall, Notification, RpcClient};
use self::session::SessionManager;
use self::state::State;
pub use self::vim::{Vim, VimProgressor};
use anyhow::{anyhow, Result};
use parking_lot::Mutex;
use serde_json::{json, Value};
use std::io::{BufReader, BufWriter};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::time::Instant;

/// Starts and keep running the server on top of stdio.
pub async fn start() {
    let (call_tx, call_rx) = tokio::sync::mpsc::unbounded_channel();

    let rpc_client = Arc::new(RpcClient::new(
        BufReader::new(std::io::stdin()),
        BufWriter::new(std::io::stdout()),
        call_tx.clone(),
    ));

    let state = State::new(call_tx, rpc_client.clone());
    let session_client = Client::new(state, rpc_client);
    session_client.loop_call(call_rx).await;
}

#[derive(Clone)]
struct Client {
    vim: Vim,
    state_mutex: Arc<Mutex<State>>,
    session_manager_mutex: Arc<Mutex<SessionManager>>,
}

impl Client {
    /// Creates a new instnace of [`Client`].
    fn new(state: State, rpc_client: Arc<RpcClient>) -> Self {
        let vim = Vim::new(rpc_client);
        Self {
            vim,
            state_mutex: Arc::new(Mutex::new(state)),
            session_manager_mutex: Arc::new(Mutex::new(SessionManager::default())),
        }
    }

    /// Entry of the bridge between Vim and Rust.
    ///
    /// Handle the message actively initiated from Vim.
    async fn loop_call(self, mut rx: UnboundedReceiver<Call>) {
        // If the debounce timer isn't active, it will be set to expire "never",
        // which is actually just 1 year in the future.
        const NEVER: Duration = Duration::from_secs(365 * 24 * 60 * 60);

        let mut pending_notification = None;
        let mut notification_dirty = false;
        let notification_delay = Duration::from_millis(50);
        let notification_timer = tokio::time::sleep(NEVER);
        tokio::pin!(notification_timer);

        loop {
            tokio::select! {
                maybe_call = rx.recv() => {
                    match maybe_call {
                        Some(call) => {
                            match call {
                                Call::Notification(notification) => {
                                    // Avoid spawn too frequently if user opens and
                                    // closes the provider frequently in a very short time.
                                    