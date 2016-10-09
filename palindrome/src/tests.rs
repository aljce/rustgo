#![cfg(test)]

#[test]
fn works() {
    assert!(super::is(String::from("racecar")));
    assert!(super::is(String::from("✓rac✓e✓car✓")));
    assert!(!super::is(String::from("✓rac✓ecar✓")));
}
