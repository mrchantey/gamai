use crate::prelude::*;
use serde::Deserialize;
use serde::Serialize;


type Foo = BuiltinNodes;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BuiltinNodes {
	PassScorer(PassScorer),
	FailScorer(FailScorer),
	SuccessAction(SuccessAction),
	FailureAction(FailureAction),
	FallbackSelector(FallbackSelector),
	SequenceSelector(SequenceSelector),
	UtilitySelector(UtilitySelector),
}

impl BuiltinNodes {
	fn into_node_struct(&self) -> &dyn NodeStruct {
		match self {
			BuiltinNodes::PassScorer(x) => x,
			BuiltinNodes::FailScorer(x) => x,
			BuiltinNodes::SuccessAction(x) => x,
			BuiltinNodes::FailureAction(x) => x,
			BuiltinNodes::FallbackSelector(x) => x,
			BuiltinNodes::SequenceSelector(x) => x,
			BuiltinNodes::UtilitySelector(x) => x,
		}
	}
}



impl NodeStruct for BuiltinNodes {
	fn init(&self, entity: &mut bevy_ecs::world::EntityWorldMut<'_>) {
		self.into_node_struct().init(entity)
	}

	fn init_from_command(&self, entity: &mut bevy_ecs::system::EntityCommands) {
		self.into_node_struct().init_from_command(entity)
	}

	fn get_sync_system(&self) -> bevy_ecs::schedule::SystemConfigs {
		self.into_node_struct().get_sync_system()
	}

	fn get_node_system(&self) -> bevy_ecs::schedule::SystemConfigs {
		self.into_node_struct().get_node_system()
	}
}
