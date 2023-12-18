use crate::prelude::*;
use bevy_ecs::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[action(system=pass_scorer)]
#[derive(Clone, Serialize, Deserialize, Component)]
pub struct PassScorer {
	#[shared]
	pub score: Score,
}

impl Default for PassScorer {
	fn default() -> Self { Self { score: Score::Pass } }
}

pub fn pass_scorer(mut query: Query<&mut PassScorer, With<Running>>) {
	for mut item in query.iter_mut() {
		item.score = Score::Pass;
	}
}

#[action(system=fail_scorer)]
#[derive(Default, Serialize, Deserialize, Clone, Component)]
pub struct FailScorer {
	#[shared]
	pub score: Score,
}

pub fn fail_scorer(mut query: Query<&mut FailScorer, With<Running>>) {
	for mut item in query.iter_mut() {
		item.score = Score::Fail;
	}
}
