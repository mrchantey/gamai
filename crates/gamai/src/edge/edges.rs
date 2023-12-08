use bevy_derive::Deref;
use bevy_derive::DerefMut;
use bevy_ecs::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Edges(pub Vec<Entity>);


impl Edges {
	pub fn new() -> Self { Self(Vec::new()) }

	pub fn with_child(mut self, edge: Entity) -> Self {
		self.push(edge);
		self
	}
}
