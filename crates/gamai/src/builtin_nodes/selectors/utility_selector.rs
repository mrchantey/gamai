use super::*;
use crate::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::SystemConfigs;
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Clone, Serialize, Deserialize, Component)]
#[node(system=utility_selector)]
pub struct UtilitySelector;

pub fn utility_selector(
	mut commands: Commands,
	selectors: Query<(Entity, &UtilitySelector, &Edges), With<Running>>,
	children_scores: Query<(Entity, &Score)>,
	children_running: Query<(), With<Running>>,
	children_results: Query<&RunResult>,
) {
	for (parent, _selector, children) in selectors.iter() {
		if any_child_running(children, &children_running) {
			continue;
		}


		if let Some((_, result)) =
			first_child_result(children, &children_results)
		{
			commands.entity(parent).insert(result.clone());
			continue;
		}

		if let Some((child, _)) = highest_score(children, &children_scores) {
			commands.entity(child).insert(Running);
		}
	}
}
