//! Regex patterns and utilities used for manipulating the line.

use once_cell::sync::Lazy;
use regex::Regex;

static GREP_POS: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(.*?):(\d+):(\d+):(.*)").unwrap());

static DUMB_JUMP_LINE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\[(.*)\](.*?):(\d+):(\d+):").unwrap());

// Match the file path and line number of grep line.
static GREP_STRIP_FPATH: Lazy<Regex> = Lazy::new(|| Regex::new(r"^.*?:\d+:\d+:").unwrap());

// Match the tag_name:lnum of tag line.
static TAG_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(.*:\d+)").unwrap());

static PROJ_TAGS: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(.*):(\d+).*\[(.*)@(.*?)\]").unwrap());

static BUFFER_TAGS: Lazy<Regex> = Lazy::new(|| Regex::new(r"^.*:(\d+).*\[(.*)\]").unwrap());

static COMMIT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^.*\d{4}-\d{2}-\d{2}\s+([0-9a-z]+)\s+").unwrap());

static GTAGS: Lazy<Regex> = Lazy::new(|| Regex::new(r"(.*)\s+(\d+)\s+(.*)").unwrap());

pub fn parse_gtags(line: &str) -> Option<(usi