mod tests;

// is returns true if s is a palindrome.
pub fn is(s: String) -> bool {
    return s
        .chars()
        .zip(s.chars().rev())
        .take(s.len() / 2)
        .all(|(a, b)| {a == b});
}
