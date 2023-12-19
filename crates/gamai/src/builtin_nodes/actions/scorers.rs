use crate::prelude::*;
use bevy_ecs::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[action(system=empty_action)]
#[derive(Clone, Serialize, Deserialize, Component)]
pub struct SetScore {
	#[shared]
	pub score: Score,
}

impl SetScore {
	pub fn new(score: Score) -> Self { Self { score } }
}