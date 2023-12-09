use crate::prelude::*;
use bevy_derive::Deref;
use bevy_derive::DerefMut;
use bevy_ecs::prelude::*;
use bevy_ecs::world::World;
use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;
use std::fmt::Debug;


#[derive(Debug, Clone, Deref, DerefMut)]
pub struct NodeGraph<'a, T>(pub DiGraph<Option<&'a T>, ()>);


impl<'a, T: Debug> NodeGraph<'a, T> {
	pub fn print_tree(&self) {
		// println!("{:?}", self);
		for node in self.0.node_indices() {
			println!("{:?}", self.0.node_weight(node));
		}
	}
}


impl<'a, T: Component> NodeGraph<'a, T> {
	pub fn new(entity: Entity, world: &'a World) -> Self {
		let mut this = Self(DiGraph::default());
		this.add_recursive(entity, world);
		this
	}

	fn add_recursive(&mut self, entity: Entity, world: &'a World) -> NodeIndex {
		let value = world.get::<T>(entity);
		let node_index = self.add_node(value);
		if let Some(children) = world.get::<Edges>(entity) {
			for child in children.iter() {
				let child_index = self.add_recursive(*child, world);
				self.add_edge(node_index, child_index, ());
			}
		}
		node_index
	}
}
