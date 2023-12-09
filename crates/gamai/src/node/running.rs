use bevy_ecs::prelude::*;
use bevy_time::Stopwatch;
use bevy_time::Time;
use std::fmt::Debug;


/// Indicate this node is currently running.
/// As this is frequently added and removed, it is `SparseSet`.
#[derive(Default, Debug, Component)]
#[component(storage = "SparseSet")]
pub struct Running;

/// Indicate the result of an action.
/// As this is frequently added and removed, it is `SparseSet`.
#[derive(Default, Debug, Clone, Copy, Component, PartialEq)]
#[component(storage = "SparseSet")]
pub enum RunResult {
	#[default]
	/// The Action was successful.
	Success,
	/// The Action was unsuccessful.
	Failure,
}

/// Tracks the last time a node was run.
#[derive(Default, Debug, Component)]
pub struct RunTimer {
	/// Last time the node was last started, or time since level load if never started.
	pub last_started: Stopwatch,
	/// Last time the node was last stopped, or time since level load if never stopped.
	pub last_stopped: Stopwatch,
}




/// Syncs [`RunTimer`] components, by default added to [`PreNodeUpdateSet`].
pub fn sync_run_timers(
	time: Res<Time>,
	mut timers: Query<&mut RunTimer>,
	added: Query<Entity, Added<Running>>,
	mut removed: RemovedComponents<Running>,
) {
	for mut timer in timers.iter_mut() {
		timer.last_started.tick(time.delta());
		timer.last_stopped.tick(time.delta());
	}
	
	for added in added.iter() {
		if let Ok(mut timer) = timers.get_mut(added) {
			timer.last_started.reset();
		}
	}
	
	for removed in removed.read() {
		if let Ok(mut timer) = timers.get_mut(removed) {
			timer.last_stopped.reset();
		}
	}
}

/// Syncs [`Running`] and [`RunResult`] components, by default added to [`PostNodeUpdateSet`].
pub fn sync_running(
	mut commands: Commands,
	// occurs immediately after `RunResult` is added
	first_pass: Query<Entity, (Added<RunResult>, With<Running>)>,
	// occurs one frame later
	second_pass: Query<Entity, (With<RunResult>, Without<Running>)>,
) {
	for entity in first_pass.iter() {
		commands.entity(entity).remove::<Running>();
	}
	for entity in second_pass.iter() {
		commands.entity(entity).remove::<RunResult>();
	}
}
