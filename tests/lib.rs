extern crate rust_guessing_game;
use rust_guessing_game::my_funcs;

#[test]
fn compaire_greater_than() {
    assert!(my_funcs::compaire(1,0) == Greater);
}

#[test]
fn compaire_less_than() {
    assert!(my_funcs::compaire(0,1) == Less);
}

#[test]
fn compaire_equal_to() {
    assert!(my_funcs::compaire(1,1) == Equal);
}
