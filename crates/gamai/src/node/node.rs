// use super::sync_run_timers;
use super::sync_running;
use crate::prelude::*;
use bevy_app::App;
use bevy_app::Update;
use bevy_derive::Deref;
use bevy_derive::DerefMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::schedule::IntoSystemSetConfigs;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_ecs::schedule::SystemConfigs;
use bevy_ecs::system::EntityCommands;
use bevy_ecs::world::EntityWorldMut;


#[derive(Debug, PartialEq, Deref, DerefMut, Component)]
pub struct TargetEntity(pub Entity);

pub trait NodeSystem: SyncSystem {
	fn get_node_system(&self) -> SystemConfigs;
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct PreNodeUpdateSet;
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct NodeUpdateSet;
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct PostNodeUpdateSet;

pub trait NodeStruct {
	fn init(&self, entity: &mut EntityWorldMut<'_>);
	fn init_from_command(&self, entity: &mut EntityCommands);
}

pub trait IntoNodes {
	fn into_nodes(self) -> Vec<Node>;
}

impl IntoNodes for Node {
	fn into_nodes(self) -> Vec<Node> { vec![self] }
}
impl<T1: IntoNode, T2: IntoNode> IntoNodes for (T1, T2) {
	fn into_nodes(self) -> Vec<Node> {
		vec![self.0.into_node(), self.1.into_node()]
	}
}

pub trait IntoNode {
	fn into_node(self) -> Node;
}
impl IntoNode for () {
	fn into_node(self) -> Node {
		Node {
			node_structs: Vec::new(),
			node_systems: Vec::new(),
			children: Vec::new(),
		}
	}
}

impl IntoNode for Node {
	fn into_node(self) -> Node { self }
}

impl<T1: IntoNode, T2: IntoNode> IntoNode for (T1, T2) {
	fn into_node(self) -> Node {
		let mut this = Node {
			node_structs: Vec::new(),
			node_systems: Vec::new(),
			children: Vec::new(),
		};

		let a = self.0.into_node();
		let b = self.1.into_node();

		this.node_structs.extend(a.node_structs);
		this.node_structs.extend(b.node_structs);
		this.node_systems.extend(a.node_systems);
		this.node_systems.extend(b.node_systems);
		this.children.extend(a.children);
		this.children.extend(b.children);
		this
	}
}

impl<T> IntoNode for T
where
	T: 'static + Clone + NodeStruct + NodeSystem,
{
	fn into_node(self) -> Node {
		Node {
			node_structs: vec![Box::new(self.clone())],
			node_systems: vec![Box::new(self)],
			children: Vec::new(),
		}
	}
}

pub struct Node {
	pub node_structs: Vec<Box<dyn NodeStruct>>,
	pub node_systems: Vec<Box<dyn NodeSystem>>,
	pub children: Vec<Node>,
}

impl Node {
	// pub fn leaf(node: impl IntoNode) -> Self { Self::branch(node, Vec::new()) }

	// pub fn branch(node: impl IntoNode, children: Vec<Node>) -> Self {
	// 	Self {
	// 		children,
	// 		node_structs: vec![Box::new(node.clone())],
	// 		node_systems: vec![Box::new(node.clone())],
	// 	}
	// }

	pub fn with_children(mut self, children: impl IntoNodes) -> Self {
		self.children = children.into_nodes();
		self
	}

	pub fn spawn_running(&self, world: &mut World, target: Entity) -> Entity {
		let root = self.spawn_graph(world, target);
		world.entity_mut(root).insert(Running);
		root
	}

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

		for node_struct in self.node_structs.iter() {
			node_struct.init(&mut entity);
		}

		entity.id()
	}

	pub fn add_systems(&self, app: &mut App) {
		app.configure_sets(Update, PreNodeUpdateSet);
		app.configure_sets(Update, NodeUpdateSet.after(PreNodeUpdateSet));
		app.configure_sets(Update, PostNodeUpdateSet.after(NodeUpdateSet));

		// app.add_systems(Update, sync_run_timers.in_set(PostNodeUpdateSet));
		app.add_systems(Update, sync_running.in_set(PostNodeUpdateSet));

		self.add_systems_to_schedule(
			app,
			Update,
			NodeUpdateSet,
			PostNodeUpdateSet,
		);
	}

	pub fn add_systems_to_schedule(
		&self,
		app: &mut App,
		schedule: impl ScheduleLabel + Clone,
		node_system_set: impl SystemSet + Clone,
		sync_system_set: impl SystemSet + Clone,
	) {
		for system in self.node_systems.iter() {
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
