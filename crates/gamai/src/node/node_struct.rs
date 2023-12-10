use bevy_ecs::schedule::SystemConfigs;
use bevy_ecs::system::EntityCommands;
use bevy_ecs::world::EntityWorldMut;

pub trait IntoNodeStruct {
	fn into_node_struct(&self) -> &dyn NodeStruct;
}

impl<T: 'static + IntoNodeStruct> NodeStruct for T {
	fn init(&self, entity: &mut bevy_ecs::world::EntityWorldMut<'_>) {
		self.into_node_struct().init(entity)
	}

	fn init_from_command(&self, entity: &mut EntityCommands) {
		self.into_node_struct().init_from_command(entity)
	}

	fn get_sync_system(&self) -> SystemConfigs {
		self.into_node_struct().get_sync_system()
	}

	fn get_node_system(&self) -> SystemConfigs {
		self.into_node_struct().get_node_system()
	}
}


pub trait NodeStruct: 'static {
	fn init(&self, entity: &mut EntityWorldMut<'_>);
	fn init_from_command(&self, entity: &mut EntityCommands);
	fn get_sync_system(&self) -> SystemConfigs;
	fn get_node_system(&self) -> SystemConfigs;
}
