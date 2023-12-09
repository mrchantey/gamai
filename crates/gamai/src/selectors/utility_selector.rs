use crate::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::SystemConfigs;

#[derive(Default, Clone, Component)]
#[node(system=utility_selector)]
pub struct UtilitySelector;

pub fn utility_selector(
	mut commands: Commands,
	selectors: Query<(&UtilitySelector, &Edges), With<Running>>,
	scores: Query<(Entity, &Score)>,
	children_running: Query<(), With<Running>>,
) {
	for (_selector, child_entities) in selectors.iter() {
		'none_running: {
			if child_entities
				.iter()
				.any(|child| children_running.contains(*child))
			{
				break 'none_running;
			}

			let highest = child_entities.iter().fold(None, |prev, child| {
				if let Ok((child, score)) = scores.get(*child) {
					if let Some((_, last_score)) = prev {
						if *score > last_score {
							return Some((child, *score));
						}
					}
				}
				prev
			});

			if let Some((entity, _)) = highest {
				commands.entity(entity).insert(Running);
			}
		}
	}
}
