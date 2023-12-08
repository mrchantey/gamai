use crate::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::SystemConfigs;

#[derive(Default, Clone, Component)]
#[node(system=utility_selector)]
pub struct UtilitySelector;

pub fn utility_selector(
	mut commands: Commands,
	selectors: Query<(&UtilitySelector, &Edges)>,
	scores: Query<(Entity, &Score)>,
	children_running: Query<(), With<Running>>,
) {
	for (_selector, child_entities) in selectors.iter() {
		'none_running: {
			for edge in child_entities.iter() {
				if children_running.get(*edge).is_ok() {
					break 'none_running;
				}

				let mut highest = None;

				if let Ok((child, score)) = scores.get(*edge) {
					let is_higher = if let Some((_, last_score)) = highest {
						*score > last_score
					} else {
						true
					};
					if is_higher {
						highest = Some((child, *score));
					}
				}

				if let Some((entity, _)) = highest {
					commands.entity(entity).insert(Running);
				}
			}
		}
	}
}
