// cannot create integration tests to test binary crate (src/main.rs)
// thus, logics should be in library crates and leave binary crate logic small
// cargo test --test integration_test
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}