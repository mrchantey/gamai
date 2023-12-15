use gamai::prelude::*;



pub fn main() {}

#[action(system=foo)]
#[derive(Clone, Serialize, Deserialize, Component)]
pub struct Foo {
	score: Score,
}

fn foo() {}
