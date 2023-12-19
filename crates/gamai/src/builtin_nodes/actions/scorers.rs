use crate::prelude::*;
use bevy_ecs::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[action(system=empty_action)]
#[derive(Clone, Serialize, Deserialize, Component)]
pub struct ScoreSetter {
	#[shared]
	pub score: Score,
}

impl ScoreSetter {
	pub fn new(score: Score) -> Self { Self { score } }
}


#[action(system=fail_scorer)]
#[derive(Default, Serialize, Deserialize, Clone, Component)]
pub struct FailScorer {
	#[shared]
	pub score: Score,
}

pub fn fail_scorer(
	mut query: Query<&mut FailScorer, (With<Running>, Added<FailScorer>)>,
) {
	for mut item in query.iter_mut() {
		item.score = Score::Fail;
	}
}
