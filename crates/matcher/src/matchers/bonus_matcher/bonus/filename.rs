use types::Score;

/// Returns a bonus score if the match indices of an item include the file name part.
///
/// Formula:
///   bonus_score = base_score * len(matched_elements_in_file_name)