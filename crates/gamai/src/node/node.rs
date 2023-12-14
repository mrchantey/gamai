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
pub struct PreTickSet;
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct TickSet;
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct PostTickSet;


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
		app.configure_sets(Update, PreTickSet);
		app.configure_sets(Update, TickSet.after(PreTickSet));
		app.configure_sets(Update, PostTickSet.after(TickSet));
		app.add_systems(
			Update,
			apply_deferred.after(TickSet).before(PostTickSet),
		);
		app.add_systems(Update, sync_running.in_set(PostTickSet));

		self.add_systems_to_schedule(
			app,
			Update,
			PreTickSet,
			TickSet,
			PostTickSet,
		);
	}

	pub fn add_systems_to_schedule(
		&self,
		app: &mut App,
		schedule: impl ScheduleLabel + Clone,
		pre_tick_set: impl SystemSet + Clone,
		tick_set: impl SystemSet + Clone,
		post_tick_set: impl SystemSet + Clone,
	) {
		for system in self.items.iter() {
			// app.add_systems(
			// 	schedule.clone(),
			// 	system.get_pre_sync_system().in_set(pre_tick_set.clone()),
			// );
			app.add_systems(
				schedule.clone(),
				system.get_node_system().in_set(tick_set.clone()),
			);
			app.add_systems(
				schedule.clone(),
				system.get_post_sync_system().in_set(post_tick_set.clone()),
			);
		}

		for child in self.children.iter() {
			child.add_systems_to_schedule(
				app,
				schedule.clone(),
				pre_tick_set.clone(),
				tick_set.clone(),
				post_tick_set.clone(),
			);
		}
	}
}
