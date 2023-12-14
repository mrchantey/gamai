pub mod builtin_nodes;
pub mod edge;
pub mod node;
pub mod message;
pub mod prop;

// allows proc macros to work internally
extern crate self as gamai;

pub mod prelude {

	pub use crate::builtin_nodes::actions::*;
	pub use crate::builtin_nodes::selectors::*;
	pub use crate::builtin_nodes::*;
	pub use crate::edge::*;
	pub use crate::message::*;
	pub use crate::node::*;
	pub use crate::prop::*;
	pub use gamai_macros::*;
}


pub mod exports {
	pub use bevy_ecs::prelude::*;
	pub use bevy_ecs::schedule::SystemConfigs;
	pub use bevy_ecs::system::EntityCommands;
	pub use serde::Deserialize;
	pub use serde::Serialize;
}
