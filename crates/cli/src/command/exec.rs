use crate::app::Args;
use anyhow::Result;
use clap::Parser;
use maple_core::process::shell_command;
use maple_core::process::{CacheableCommand, ShellCommand};
use std::path::PathBuf;
use std::process::Command;

/// Execute the shell command
#[derive(Parser, Debug, Clone)]
pub struct Exec {
    /// Specify the system command to run.
    #[clap(index = 1)]
    shell_cmd: String,

    /// Specify the working directory of CMD
    #[clap(long, parse(from_os_str))]
    cmd_dir: Option<PathBuf>,

    /// Specify the threshold for writing the output of command to a tempfile.
    #[clap(long, default_value = "100000")]
    output_threshold: usize,
}

impl Exec {
    // This can work with the piped command, e.g., git ls-files | uniq.
    fn prepare_exec_cmd(&self) -> Command {
        let mut cmd = shell_command(self.shell_cmd.as_str());

        if let Some(ref cmd_dir) = self.cmd_dir {
            cmd.current_dir(cmd_dir);
        }

        cmd
    }

    pub fn run(
        &self,
        Args {
            number,
            icon,
            no_cache,
            ..
        }: Args,
    ) -> Result<()> {
        let mut exec_cmd = self.prepare_exec_cmd();

        // TODO: fix this properly
        //
        // `let g:clap_builtin_fuzzy_filter_threshold == 0` is used to configure clap always use
        // the async on_typed impl, but some commands also makes this variable to control
        // `--output-threshold`, which can be problamatic. I imagine not many people act