use crate::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::SystemConfigs;
use bevy_ecs::system::EntityCommands;
use bevy_ecs::world::EntityWorldMut;


#[typetag::serde]
pub trait Action {
	fn duplicate(&self) -> Box<dyn Action>;

	fn spawn(&self, entity: &mut EntityWorldMut<'_>);
	fn spawn_with_command(&self, entity: &mut EntityCommands);

	// fn pre_tick_system(&self) -> SystemConfigs;
	fn tick_system(&self) -> SystemConfigs;
	fn post_tick_system(&self) -> SystemConfigs;

	fn prop_listeners(&self, entity: Entity) -> Vec<SetBevyProp>;
}
