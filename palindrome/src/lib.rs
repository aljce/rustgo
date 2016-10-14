mod tests;

use std::str::Chars;

// is returns true if s is a palindrome.
pub fn is(s: String) -> bool {
    is_iter(exact_size_chars(&s))
}

// Rust strings require a full traversal to find length so this isnt in the std library
#[derive(Clone)]
pub struct ExactSizeChars<'a> {
    chars: Chars<'a>,
    size:  usize
}

pub fn exact_size_chars<'a>(string: &'a str) -> ExactSizeChars<'a> {
    ExactSizeChars {
        chars: string.chars(),
        size:  string.chars().count()
    }
}

impl<'a> Iterator for ExactSizeChars<'a> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        self.chars.next()
    }
}

impl<'a> DoubleEndedIterator for ExactSizeChars<'a> {
    fn next_back(&mut self) -> Option<char> {
        self.chars.next_back()
    }
}

impl<'a> ExactSizeIterator for ExactSizeChars<'a> {
    fn len(&self) -> usize {
        self.size
    }
}

// This palindrom function is now rusty
// Thing is ExactSizeIterators are rare and dont even exist for Strings as
// chars can be 1 - 4 bytes long. This means that String length takes O(n) time.
pub fn is_iter<I>(iter: I) -> bool
    where I: Iterator + DoubleEndedIterator + ExactSizeIterator + Clone,
          I::Item: PartialEq, {
    let mid_point = iter.len() / 2;
    let reversed  = iter.clone().rev();
    iter.zip(reversed).take(mid_point).all(|(a,b)| a == b)
}
