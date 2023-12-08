use crate::prelude::*;
use bevy_ecs::prelude::*;

#[node(system=score_always_pass)]
#[derive(Clone, Component)]
pub struct ScoreAlwaysPass {
	score: Score,
}

impl Default for ScoreAlwaysPass {
	fn default() -> Self { Self { score: Score::Pass } }
}

pub fn score_always_pass(
	mut query: Query<&mut ScoreAlwaysPass, With<Running>>,
) {
	for mut item in query.iter_mut() {
		item.score = Score::Pass;
	}
}

#[node(system=score_always_fail)]
#[derive(Default, Clone, Component)]
pub struct ScoreAlwaysFail {
	score: Score,
}

pub fn score_always_fail(
	mut query: Query<&mut ScoreAlwaysFail, With<Running>>,
) {
	for mut item in query.iter_mut() {
		item.score = Score::Fail;
	}
}
