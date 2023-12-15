use crate::prelude::*;
use anyhow::anyhow;
use anyhow::Result;
use bevy_derive::Deref;
use bevy_derive::DerefMut;
use bevy_ecs::prelude::*;
use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;

pub type ActionList = Vec<Box<dyn Action>>;
pub type ActionTree = Tree<ActionList>;

impl ActionTree {
	pub fn from_action(value: impl Action) -> Self {
		Self {
			value: vec![Box::new(value)],
			children: vec![],
		}
	}
	pub fn with_child(mut self, child: Self) -> Self {
		self.children.push(child);
		self
	}
	pub fn into_graph(self) -> ActionGraph { ActionGraph::from_tree(self) }
}

#[derive(Default, Deref, DerefMut)]
pub struct ActionGraph(pub DiGraph<ActionList, ()>);

impl Clone for ActionGraph {
	fn clone(&self) -> Self {
		let graph = map_graph(&self.0, |_, action_list| {
			action_list
				.into_iter()
				.map(|action| action.duplicate())
				.collect::<Vec<_>>()
		});
		ActionGraph(graph)
	}
}

impl ActionGraph {
	pub fn new() -> Self { Self::default() }

	pub fn from_tree(root: impl Into<ActionTree>) -> Self {
		let mut this = Self::new();
		this.add_recursive(root.into());
		this
	}

	pub fn add_recursive(&mut self, tree: ActionTree) -> NodeIndex {
		let ActionTree { value, children } = tree;
		let node = self.add_node(value);

		for child in children {
			let index = self.add_recursive(child);
			self.add_edge(node, index, ());
		}

		node
	}

	pub fn spawn(&self, world: &mut World, target: Entity) -> EntityGraph {
		// create entities & actions
		let entity_graph = map_graph(&self.0, |_, actions| {
			let mut entity =
				world.spawn((TargetEntity(target), RunTimer::default()));

			for action in actions.iter() {
				action.spawn(&mut entity);
			}
			entity.id()
		});

		// create edges
		for (index, entity) in Iterator::zip(
			entity_graph.node_indices(),
			entity_graph.node_weights(),
		) {
			let children = entity_graph
				.neighbors_directed(index, petgraph::Direction::Outgoing)
				.map(|index| entity_graph[index])
				.collect::<Vec<_>>();
			world.entity_mut(*entity).insert(Edges(children));
		}

		EntityGraph(entity_graph)
	}
}

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct EntityGraph(pub DiGraph<Entity, ()>);

impl EntityGraph {
	pub fn set_action(
		&self,
		world: &mut World,
		message: &SetActionMessage,
	) -> Result<()> {
		let mut entity = self
			.0
			.node_weight(NodeIndex::new(*message.index))
			.map(|entity| world.entity_mut(*entity))
			.ok_or_else(|| anyhow!("Node not found: {}", *message.index))?;

		message.value.spawn(&mut entity);
		Ok(())
	}
	pub fn set_action_with_command(
		&self,
		commands: &mut Commands,
		message: &SetActionMessage,
	) -> Result<()> {
		let entity = self
			.0
			.node_weight(NodeIndex::new(*message.index))
			.ok_or_else(|| anyhow!("Node not found: {}", *message.index))?;

		let mut entity = commands
			.get_entity(*entity)
			.ok_or_else(|| anyhow!("Entity not found: {}", *message.index))?;

		message.value.spawn_with_command(&mut entity);
		Ok(())
	}
}
