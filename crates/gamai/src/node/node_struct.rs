use bevy_ecs::schedule::SystemConfigs;
use bevy_ecs::system::EntityCommands;
use bevy_ecs::world::EntityWorldMut;

pub trait NodeStruct: 'static + NodeStructMeta {
	fn init(&self, entity: &mut EntityWorldMut<'_>);
	fn init_from_command(&self, entity: &mut EntityCommands);
	fn get_pre_sync_system(&self) -> SystemConfigs;
	fn get_node_system(&self) -> SystemConfigs;
	fn get_post_sync_system(&self) -> SystemConfigs;
}

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
	fn get_pre_sync_system(&self) -> SystemConfigs {
		self.into_node_struct().get_pre_sync_system()
	}
	fn get_node_system(&self) -> SystemConfigs {
		self.into_node_struct().get_node_system()
	}

	fn get_post_sync_system(&self) -> SystemConfigs {
		self.into_node_struct().get_post_sync_system()
	}
}

impl<T: 'static + IntoNodeStruct> NodeStructMeta for T {
	fn name(&self) -> &'static str { self.into_node_struct().name() }
}

pub trait NodeStructVariants {
	fn get_node_struct_variants() -> Vec<Box<dyn NodeStruct>>;
}

impl<T: NodeStruct + Default> NodeStructVariants for T {
	fn get_node_struct_variants() -> Vec<Box<dyn NodeStruct>> {
		vec![Box::new(T::default())]
	}
}


pub trait NodeStructMeta {
	fn name(&self) -> &'static str;
	// fn description() -> &'static str;
}
