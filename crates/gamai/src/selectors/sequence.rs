use crate::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::SystemConfigs;

#[derive(Default, Clone, Component)]
#[node(system=sequence)]
pub struct SequenceSelector;
/// A node that runs all of its children in order until one fails.
///
/// If a child succeeds it will run the next child.
///
/// If there are no more children to run it will succeed.
///
/// If a child fails it will fail.
pub fn sequence(
	mut commands: Commands,
	selectors: Query<(Entity, &SequenceSelector, &Edges), With<Running>>,
	children_running: Query<(), With<Running>>,
	children_results: Query<&RunResult>,
) {
	for (parent, _selector, child_entities) in selectors.iter() {
		'none_running: {
			if child_entities
				.iter()
				.any(|child| children_running.contains(*child))
			{
				break 'none_running;
			}

			let first_result =
				child_entities
					.iter()
					.enumerate()
					.find_map(|(index, child)| {
						if let Ok(result) = children_results.get(*child) {
							Some((index, result))
						} else {
							None
						}
					});

			match first_result {
				Some((index, result)) => match result {
					&RunResult::Failure => {
						commands.entity(parent).insert(RunResult::Failure);
					}
					&RunResult::Success => {
						if index == child_entities.len() - 1 {
							commands.entity(parent).insert(RunResult::Success);
						} else {
							commands
								.entity(child_entities[index + 1])
								.insert(Running);
						}
					}
				},
				None => {
					commands.entity(child_entities[0]).insert(Running);
				}
			}
		}
	}
}
