use crate::prelude::*;
use bevy_ecs::prelude::*;

#[node(system=always_run_result_success)]
#[derive(Default, Clone, Component)]
pub struct AlwaysRunResultSuccess;

pub fn always_run_result_success(
	mut commands: Commands,
	mut query: Query<Entity, (With<AlwaysRunResultSuccess>, With<Running>)>,
) {
	for entity in query.iter_mut() {
		commands.entity(entity).insert(RunResult::Success);
	}
}

#[node(system=always_run_result_failure)]
#[derive(Default, Clone, Component)]
pub struct AlwaysRunResultFailure;

pub fn always_run_result_failure(
	mut commands: Commands,
	mut query: Query<Entity, (With<AlwaysRunResultFailure>, With<Running>)>,
) {
	for entity in query.iter_mut() {
		commands.entity(entity).insert(RunResult::Failure);
	}
}
