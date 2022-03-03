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
        acc_width[match_start - 1]
    };

    let w2 = if match_end >= acc_width.len() {
        full_width - w1
    } else {
        acc_width[match_end] - w1
    };

    let w3 = full_width - w1 - w2;

    if (w1 > w3 && w2 + w3 <= container_width) || (w3 <= 2) {
        // right-fixed, ..ring
        let (trimmed_text, trimmed_len) = trim_left(text, container_width - 2, tabstop);

        let text = format!("..{trimmed_text}");
        let indices = indices
            .iter()
            .filter_map(|x| (x + 2).checked_sub(trimmed_len))
            .filter(|x| *x > 1) // Ignore the highlights in `..`
            .collect();

        Some((text, indices))
    } else if w1 <= w3 && w1 + w2 <= container_width {
        // left-fixed, Stri..
        let trimmed_text = trim_right(text, container_width - 2, tabstop);

        let text = format!("{trimmed_text}..");
        let indices = indices
            .iter()
            .filter(|x| *x + 2 < container_width) // Ignore the highlights in `..`
            .copied()
            .collect::<Vec<_>>();

        Some((text, indices))
    } else {
        // Convert the char-position to byte-position.
        let match_start_byte_idx = text.char_indices().nth(match_start)?.0;

        // left-right, ..Stri..
        let left_truncated_text = &text[match_start_byte_idx..];
        let trimmed_text = trim_right(left_truncated_text, container_width - 2 - 2, tabstop);

        let text = format!("..{trimmed_text}..");
        let indices = indices
            .iter()
            .map(|x| x - match_start + 2)
            .filter(|x| *x + 2 < container_width) // Ignore the highlights in `..`
            .collect::<Vec<_>>();

        Some((text, indices))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::filter_single_line;
    use types::MatchedItem;

    #[test]
    fn test_trim_left() {
        let text = "0123456789abcdef";
        let width = 5;
        let (trimmed, offset) = trim_left(text, width, 4);
        assert_eq!(trimmed, "bcdef");
        assert_eq!(offset, 11);
    }

    #[test]
    fn test_trim_right() {
        let text = "0123456789abcdef";
        let width = 5;
        let trimmed = trim_right(text, width, 4);
        assert_eq!(trimmed, "01234");
    }

    #[test]
    fn test_trim_text() {
        // raw_line, query, highlighted, container_width, display_line
        let test_cases = vec![
            (
                "directories/are/nested/a/lot/then/the/