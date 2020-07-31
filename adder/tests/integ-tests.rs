use adder;
mod common;

#[test]
fn integ_adds_two()
{
    common::setup();
    assert_eq!(4, adder::add_two(2));
}