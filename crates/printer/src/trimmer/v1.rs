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
        // thread 'main' panicked at 'byte index 62 is not a char boundary; it is inside '💔' (bytes 61..65) of `292                             tracing::error!(error = ?e, "💔 Error at initializing GTAGS, attempting to recreate...");`', library/core/src/str/mod.rs:127:5
        //
        // Can not use `(String::from(&text[diff..]), diff)` due to diff could not a char boundary.
        let mut chars = text.chars();
        (0..diff).for_each(|_| {
            chars.next();
        });
        (chars.as_str(), diff)
    } else {
        (text, 0)
    };

    let mut current_width = display_width(text, tabstop);

    while current_width > width && !text.is_empty() {
        text = remove_first_char(text);
        trimmed_char_len += 1;
        current_width = display_width(text, tabstop);
    }

    (text, trimmed_char_len)
}

/// `String` -> `Stri..`.
fn trim_right(text: &str, width: usize, tabstop: usize) -> &str {
    let current_width = display_width(text, tabstop);

    if current_width > width {
        if text.is_char_boundary(width) {
            &text[..width]
        } else {
            let mut width = width;
            while !text.is_char_boundary(width) {
                width -= 1;
            }
            &text[..width]
        }
    } else {
        text
    }
}

/// Returns the potential trimmed text.
///
/// In order to make the highlights of matches visible in the container as much as possible,
/// both the left and right of the original text can be trimmed.
///
/// For example, if the matches appear in the end of a long string, we should trim the left and
/// only show the right part.
///
/// ```text
/// xxxxxxxxxxxxxxxxxxxxxxxxxxMMxxxxxMxxxxx
///               shift ->|               |
/// ```
///
/// container_width = winwidth - prefix_length
///
/// # Arguments
///
/// - `text`: original untruncated text.
/// - `indices`: highlights in char-positions.
/// - `container_width`: the width of window to display the text.
pub fn trim_text(
    text: &str,
    indices: &[usize],
    container_width: usize,
    tabstop: usize,
) -> Option<(String, Vec<usize>)> {
    let match_start = indices[0];
    let match_end = *indices.last()?;

    let acc_width = accumulate_text_width(text, tabstop);

    // Width needed for diplaying the whole text.
    let full_width = *acc_width.last()?;

    if full_width <= container_width {
        return None;
    }

    //  xxxxxxxxxxxxxxxxxxxxMMxxxxxMxxxxxMMMxxxxxxxxxxxx
    // |<-      w1       ->|<-    w2     ->|<-  w3   ->|
    //
    // w1, w2, w3 = len_before_matched, len_matched, len_after_matched
    let w1 = if match_start == 0 {
        0
    } else {
        acc_widt