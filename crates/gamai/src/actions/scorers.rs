use crate::prelude::*;
use bevy_ecs::prelude::*;

#[node(system=pass_scorer)]
#[derive(Clone, Component)]
pub struct PassScorer {
	score: Score,
}

impl Default for PassScorer {
	fn default() -> Self { Self { score: Score::Pass } }
}

pub fn pass_scorer(mut query: Query<&mut PassScorer, With<Running>>) {
	for mut item in query.iter_mut() {
		item.score = Score::Pass;
	}
}

#[node(system=fail_scorer)]
#[derive(Default, Clone, Component)]
pub struct FailScorer {
	score: Score,
}

pub fn fail_scorer(mut query: Query<&mut FailScorer, With<Running>>) {
	for mut item in query.iter_mut() {
		item.score = Score::Fail;
	}
}
