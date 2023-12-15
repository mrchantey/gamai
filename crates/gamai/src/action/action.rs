use crate::prelude::*;
use anyhow::Result;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::SystemConfigs;
use bevy_ecs::system::EntityCommands;
use bevy_ecs::world::EntityWorldMut;


#[typetag::serde]
pub trait Action: 'static {
	fn duplicate(&self) -> Box<dyn Action>;

	fn spawn(&self, entity: &mut EntityWorldMut<'_>);
	fn spawn_with_command(&self, entity: &mut EntityCommands);

	// fn pre_tick_system(&self) -> SystemConfigs;
	fn tick_system(&self) -> SystemConfigs;
	fn post_tick_system(&self) -> SystemConfigs;

	fn prop_listeners(&self, entity: Entity) -> Vec<SetBevyProp>;
}

pub type SetActionFunc = Box<dyn Fn(&mut EntityCommands) -> Result<()>>;

pub trait SetAction: Action {
	fn set(&mut self, func: SetActionFunc);
}
