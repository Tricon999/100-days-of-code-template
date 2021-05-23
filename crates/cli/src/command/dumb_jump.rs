//! Inspired by https://github.com/jacktasia/dumb-jump/blob/master/dumb-jump.el.
//!
//! This module requires the executable rg with `--json` and `--pcre2` is installed in the system.

use anyhow::Result;
use clap::Parser;
use maple_core::find_usages::{CtagsSearcher, QueryType, RegexSearcher, UsageMatcher, Usages};
use maple_core::tools::ctags::{get_language, TagsGenerator};
use std::path::PathBuf;

/// Search-based jump.
#[derive(Parser, Debug, Clone)]
pub struct DumbJump {
    /// Search term.
    #[clap(index = 1)]
    pub word: String,

    /// File extension.
    #[clap(index = 2)]
    pub extension: String,

    /// Definition kind.
    #[clap(long)]
    pub kind: Option<String>,

    /// Specify the working directory.
    #[clap(long, parse(from_os_str))]
    pub cmd_dir: Option<PathBuf>,

    /// Use RegexSearcher instead of CtagsSearcher
    #[clap(long)]
    pub regex: bool,
}

impl DumbJump {
    pub fn run(self) -> Result<()> {
        let Self {
            word,
            extension,
            cmd_dir,
            ..
        } = self;

        if self.regex {
            let regex_searcher = RegexSearcher {
                word,
                extension,
                dir: cmd_dir,
            };
            let usages = regex_searcher.cli_usages(&Default::default())?;
            let total = usages.len();
            let (lines, indices): (Vec<_>, Vec<_>) = usages
                .into_iter()
                .map(|usage| (usage.line, usage.indices))
                .unzip();
            printer::println_json_with_length!(total, lines, indices);
        } else {
            let cwd = match cmd_dir {
                Some(cwd) => cwd,
                None => std::env::current_dir()?,
            };
       