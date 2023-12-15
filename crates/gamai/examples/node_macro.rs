use gamai::prelude::*;



pub fn main() {}

#[action(system=foo)]
#[derive(Clone, Component)]
pub struct Foo {
	score: Score,
}

fn foo() {}
