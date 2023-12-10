// use super::sync_run_timers;
use super::sync_running;
use crate::prelude::*;
use bevy_app::App;
use bevy_app::Update;
use bevy_derive::Deref;
use bevy_derive::DerefMut;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;

#[derive(Debug, PartialEq, Deref, DerefMut, Component)]
pub struct TargetEntity(pub Entity);

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct PreRunSet;
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct RunSet;
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct PostRunSet;


pub struct Node {
	pub items: Vec<Box<dyn NodeStruct>>,
	pub children: Vec<Node>,
}

impl Node {
	pub fn with_children(mut self, children: impl IntoNodes) -> Self {
		self.children = children.into_nodes();
		self
	}

	/// Spawn a node graph for the given target entity.
	/// The [`Running`] component is added to the root.
	pub fn spawn(&self, world: &mut World, target: Entity) -> Entity {
		let root = self.spawn_graph(world, target);
		world.entity_mut(root).insert(Running);
		root
	}

	/// Spawn a node graph for the given target entity.
	/// No additional components are added.
	pub fn spawn_graph(&self, world: &mut World, target: Entity) -> Entity {
		let edges = self
			.children
			.iter()
			.map(|child| child.spawn_graph(world, target))
			.collect::<Vec<_>>();

		let mut entity =
			world.spawn((TargetEntity(target), RunTimer::default()));

		if edges.len() > 0 {
			entity.insert(Edges(edges));
		}

		for node_struct in self.items.iter() {
			node_struct.init(&mut entity);
		}

		entity.id()
	}

	pub fn add_systems(&self, app: &mut App) {
		app.configure_sets(Update, PreRunSet);
		app.configure_sets(Update, RunSet.after(PreRunSet));
		app.configure_sets(Update, PostRunSet.after(RunSet));
		app.add_systems(
			Update,
			apply_deferred.after(RunSet).before(PostRunSet),
		);
		app.add_systems(Update, sync_running.in_set(PostRunSet));

		self.add_systems_to_schedule(app, Update, RunSet, PostRunSet);
	}

	pub fn add_systems_to_schedule(
		&self,
		app: &mut App,
		schedule: impl ScheduleLabel + Clone,
		node_system_set: impl SystemSet + Clone,
		sync_system_set: impl SystemSet + Clone,
	) {
		for system in self.items.iter() {
			app.add_systems(
				schedule.clone(),
				system.get_node_system().in_set(node_system_set.clone()),
			);
			app.add_systems(
				schedule.clone(),
				system.get_sync_system().in_set(sync_system_set.clone()),
			);
		}

		for child in self.children.iter() {
			child.add_systems_to_schedule(
				app,
				schedule.clone(),
				node_system_set.clone(),
				sync_system_set.clone(),
			);
		}
	}
}
