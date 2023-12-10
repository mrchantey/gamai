pub mod actions;
pub mod edge;
pub mod node;
pub mod prop;
pub mod selectors;

// allows proc macros to work internally
extern crate self as gamai;

pub mod prelude {

	pub use crate::actions::*;
	pub use crate::edge::*;
	pub use crate::node::*;
	pub use crate::prop::*;
	pub use crate::selectors::*;
	pub use gamai_macros::*;
}


pub mod exports {
	pub use bevy_ecs::prelude::*;
	pub use bevy_ecs::schedule::SystemConfigs;
	pub use bevy_ecs::system::EntityCommands;
}
