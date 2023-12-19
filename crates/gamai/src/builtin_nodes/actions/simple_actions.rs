use crate::prelude::*;
use bevy_ecs::prelude::*;
use bevy_utils::Duration;
use serde::Deserialize;
use serde::Serialize;


#[action(system=empty_action)]
#[derive(Default, Clone, Component, Serialize, Deserialize)]
pub struct EmptyAction;
pub fn empty_action() {}

// intentionally dont deref to avoid bugs.
#[action(system=set_run_result)]
#[derive(Default, Clone, Serialize, Deserialize, Component)]
pub struct RunResultSetter(pub RunResult);

impl RunResultSetter {
	pub fn new(result: RunResult) -> Self { Self(result) }
}


pub fn set_run_result(
	mut commands: Commands,
	mut query: Query<(Entity, &RunResultSetter), With<Running>>,
) {
	for (entity, result) in query.iter_mut() {
		commands.entity(entity).insert(result.0);
	}
}

#[action(system=succeed_in_duration)]
#[derive(Default, Clone, Serialize, Deserialize, Component)]
pub struct SucceedInDuration {
	pub duration: Duration,
}

pub fn succeed_in_duration(
	mut _commands: Commands,
	mut _query: Query<Entity, (With<RunResultSetter>, With<Running>)>,
) {
	todo!()
}
