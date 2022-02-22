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

pub fn parse_gtags(line: &str) -> Option<(usize, &str, &str)> {
    let cap = GTAGS.captures(line)?;
    let lnum = cap.get(2).map(|x| x.as_str()).and_then(parse_lnum)?;
    let path_and_pattern = cap.get(3).map(|x| x.as_str())?;
    if let Some((path, pattern)) = path_and_pattern.split_once(' ') {
        Some((lnum, path, pattern))
    } else {
        None
    }
}

/// Extract tag name from the line in tags provider.
#[inline]
pub fn extract_tag_name(line: &str) -> Option<&str> {
    TAG_RE.find(line).map(|x| x.as_str())
}

/// Returns the line content only and offset in the raw line.
///
/// Do not match the file path when using ripgrep.
///
/// //                                <----       line content       ---->
/// // crates/printer/src/lib.rs:199:26:        let query = "srlisrlisrsr";
/// //                                |
/// //                             offset
#[inline]
pub fn extract_grep_pattern(line: &str) -> Option<(&str, usize)> {
    GREP_STRIP_FPATH
        .find(line)
        .map(|mat| (&line[mat.end()..], mat.end()))
}

/// Returns a tuple of (fpath, lnum, col).
pub fn extract_grep_position(line: &str) -> Option<(&str, usize, usize, &str)> {
    let cap = GREP_POS.captures(line)?;
    let fpath = cap.get(1).map(|x| x.as_str())?;
    let str2nr = |idx: usize| cap.get(idx).map(|x| x.as_str()).and_then(parse_lnum);
    let lnum = str2nr(2)?;
    let col = str2nr(3)?;
    let line_content = cap.get(4).map(|x| x.as_str())?;
    Some((fpath, lnum, col, line_content))
}

/// Returns a tuple of (end_of_path, start_of_line).
pub fn parse_grep_item(line: &str) -> Option<(usize, usize)> {
    GREP_STRIP_FPATH.find(line).and_then(|mat| {
        let line_offset = mat.end();

        let path_lnum_col = &line[..line_offset - 1];
        match path_lnum_col.rfind(':') {
            Some(path_lnum_offset) => {
                let path_lnum = &line[..path_lnum_offset];
                path_lnum
                    .rfind(':')
                    .map(|end_of_path| (end_of_path, line_offset))
            }
            None => None,
        }
    })
}

/// Returns a tuple of (fpath, lnum, col).
pub fn extract_jump_line_info(line: &str) -> Option<(&str, &str, usize, usize)> {
    let cap = DUMB_JUMP_LINE.captures(line)?;
    let def_kind = cap.get(1).map(|x| x.as_str())?;
    let fpath = cap.get(2).map(|x| x.as_str())?;
    let str2nr = |idx: usize| cap.get(idx).map(|x| x.as_str()).a