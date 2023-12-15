use crate::prelude::*;
use bevy_ecs::prelude::*;
use serde::Deserialize;
use serde::Serialize;


#[action(system=empty_action)]
#[derive(Default, Clone, Component, Serialize, Deserialize)]
pub struct EmptyAction;
pub fn empty_action() {}

#[action(system=success_action)]
#[derive(Default, Clone, Serialize, Deserialize, Component)]
pub struct SuccessAction;

pub fn success_action(
	mut commands: Commands,
	mut query: Query<Entity, (With<SuccessAction>, With<Running>)>,
) {
	for entity in query.iter_mut() {
		commands.entity(entity).insert(RunResult::Success);
	}
}

#[action(system=failure_action)]
#[derive(Default, Clone, Serialize, Deserialize, Component)]
pub struct FailureAction;

pub fn failure_action(
	mut commands: Commands,
	mut query: Query<Entity, (With<FailureAction>, With<Running>)>,
) {
	for entity in query.iter_mut() {
		commands.entity(entity).insert(RunResult::Failure);
	}
}
