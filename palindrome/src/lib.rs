mod tests;

// is returns true if s is a palindrome.
pub fn is(s: String) -> bool {
    let forward = s.chars();
    let backward = s.chars().rev();

    return forward.zip(backward).take(s.len() / 2).all(|(a, b)| {a == b});
}
