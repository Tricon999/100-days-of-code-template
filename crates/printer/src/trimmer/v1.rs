use unicode_width::UnicodeWidthChar;

/// Returns the displayed width in columns of a `text`.
fn display_width(text: &str, tabstop: usize) -> usize {
    let mut w = 0;
    for ch in text.chars() {
        w += if ch == '\t' {
            tabstop - (w % tabstop)
        } else {
            ch.width().unwrap_or(2)
        };
    }
    w
}

/// Return an array in which arr[i] stores the display width till char[i] for `text`.
fn accumulate_text_width(text: &str, tabstop: usize) -> Vec<usize> {
    let mut ret = Vec::with_capacity(text.chars().count());
    let mut w = 0;
    for ch in text.chars() {
        w += if ch == '\t' {
            tabstop - (w % tabstop)
        } else {
            ch.width().unwrap_or(2)
        };
        ret.push(w);
    }
    ret
}

fn remove_first_char(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.as_str()
}

/// `String` -> `..ring`.
///
/// Returns the original text with the left part trimmed and the length of trimmed text in chars.
fn trim_left(text: &str, width: usize, tabstop: usize) -> (&str, usize) {
    // Assume each char takes at least one column
    let chars_count = text.chars().count();
    let (mut text, mut trimmed_char_len) = if chars_count > width + 2 {
        let diff = chars_count - width - 2;
        // 292                             tracing::error!(error = ?e, "💔 Error at initializing GTAGS, attempting to recreate...");, 56, 4
        // thread 'main' panicked at 'byte index 62 is not a char boundary; it is inside '💔' (bytes 61..65) of `292                             tracing::error!(error = ?e, "💔 Error at initializing GTAGS, attempting to recreate...");`', library/core/src/str/mod.rs:127