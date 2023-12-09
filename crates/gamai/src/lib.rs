pub mod actions;
pub mod prop;

pub mod selectors;

pub mod edge;
pub mod node;

// allows proc macros to work internally
extern crate self as gamai;

pub mod prelude {

	pub use crate::edge::Edges;
	pub use crate::node::IntoNode;
	pub use crate::node::IntoNodes;
	pub use crate::node::Node;
	pub use crate::node::NodeStruct;
	pub use crate::node::NodeSystem;
	pub use crate::node::RunResult;
	pub use crate::node::RunTimer;
	pub use crate::node::Running;
	pub use crate::node::Score;
	pub use crate::node::TargetEntity;
	pub use crate::prop::Prop;
	pub use crate::prop::SyncSystem;
	pub use crate::selectors::SequenceSelector;
	pub use crate::selectors::UtilitySelector;
	pub use gamai_macros::child_props;
	pub use gamai_macros::node;
}


pub mod exports {
	pub use bevy_ecs::prelude::*;
	pub use bevy_ecs::schedule::SystemConfigs;
	pub use bevy_ecs::system::EntityCommands;
}
