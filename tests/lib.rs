extern crate rust_guessing_game;
use rust_guessing_game::my_funcs;
use std::cmp::Ordering;

#[test]
fn compare_greater_than() {
    assert!(my_funcs::compare(1,0) == Ordering::Greater);
}

#[test]
fn compare_less_than() {
    assert!(my_funcs::compare(0,1) == Ordering::Less);
}

#[test]
fn compare_equal_to() {
    assert!(my_funcs::compare(1,1) == Ordering::Equal);
}
