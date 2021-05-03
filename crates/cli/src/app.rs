use crate::command;
use anyhow::Result;
use clap::Parser;
use filter::FilterContext;
use icon::Icon;
use types::CaseMatching;

#[derive(Parser, Debug)]
pub enum RunCmd {
    /// Start the stdio-based service, currently there is only filer support.
    #[clap(name = "rpc")]
    Rpc(command::rpc::Rpc),
    #[clap(name = "grep")]
    Grep(command::grep::Grep),
    /// Execute the ripgrep command to avoid the escape issue
    #[clap(name = "live-grep")]
    LiveGrep(command::grep::LiveGrep),
    #[clap(name = "gtags")]
    Gtags(command::gtags::Gtags),
    /// Execute the shell command.
    #[clap(name = "exec")]
    Exec(command::exec::Exec),
    /// Dumb jump.
    #[clap(name = "dumb-jump")]
    DumbJump(command::dumb_jump::DumbJump),
    /// Generate the project-wide tags using ctags.
    #[clap(name = "ctags", subcommand)]
    Ctags(command::ctags::Ctags),
    /// Interact with the cache info.
    #[clap(name = "cache", subcommand)]
    Cache(command::cache::Cache),
    /// F