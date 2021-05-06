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
    /// Fuzzy filter the input.
    #[clap(name = "filter")]
    Filter(command::filter::Filter),
    /// Filter against current Vim buffer.
    #[clap(name = "blines")]
    Blines(command::blines::Blines),
    /// Generate vim help tags.
    #[clap(name = "helptags")]
    Helptags(command::helptags::Helptags),
    /// Start the forerunner job of grep.
    #[clap(name = "ripgrep-forerunner")]
    RipGrepForerunner(command::grep::RipGrepForerunner),
}

/// Maple core CLI arguments.
#[derive(Parser, Debug)]
pub struct Args {
    /// Print the top NUM of filtered items.
    ///
    /// The returned JSON has three fields:
    ///   - total: total number of initial filtered result set.
    ///   - lines: text lines used for displaying directly.
    ///   - indices: the indices of matched elements per line, used for the highlight purpose.
    #[clap(long, name = "NUM")]
    pub number: Option<usize>,

    /// Width of clap window.
    #[clap(long)]
    pub winwidth: Option<usize>,

    /// Prepend an icon for item of files and grep provider, valid only when --number is used.
    #[clap(long, parse(from_str), default_value = "unknown")]
    pub icon: Icon,

    /// Case matching strategy.
    #[clap(long, parse(from_str), default_value = "smart")]
    pub case_matching: CaseMatching,

    /// Do not use the cached file for exec subcommand.
    #[clap(long)]
    pub no_cache: bool,

    /// Specify the number of threads used in the rayon global thread pool.
    ///
    /// By default, the number of physical cores will be used if the environment variable
    /// `RAYON_NUM_THREADS` also does not exist.
    #[clap(long)]
    pub rayon_num_threads: Option<usize>,

    /// Enable the logging system.
    #[clap(long, parse(from_os_str))]
    pub log: Option<std::path::PathBuf>,

    /// Specify the path of the config file.
    #[clap(long, parse(from_os_str))]
    pub config_file: Option<std::path::PathBuf>,
}

impl Args {
    pub fn into_filter_context(self) -> FilterContext {
        FilterContext::default()
            .icon(self.icon)
            .number(self.number)
            .winwidth(self.winwidth)
    }
}

impl RunCmd {
    pub async fn run(self, args: Args) -> Result<()> {
        // Set the global thread pool to use the number of physical cores if `RAYON_NUM_THREADS`
        // does not exist.
        //
        // > By default, Rayon uses the same number of t