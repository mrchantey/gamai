use gamai::prelude::*;



pub fn main() {}

#[node(system=foo)]
#[derive(Clone, Component)]
pub struct Foo {
	score: Score,
}

fn foo() {}
