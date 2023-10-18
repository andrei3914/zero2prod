use unicode_segmentation::UnicodeSegmentation;

pub fn is_valid_input_string(s: &str, length: usize) -> bool {
    let is_empty_or_whitespace = s.trim().is_empty();

    let is_too_long = s.graphemes(true).count() > length;

    let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_chars = s.chars().any(|g| forbidden_chars.contains(&g));

    if is_empty_or_whitespace || is_too_long || contains_forbidden_chars {
        return false;
    } else {
        return true;
    }
}
