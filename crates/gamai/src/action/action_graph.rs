use crate::prelude::*;
use bevy_derive::Deref;
use bevy_derive::DerefMut;
use bevy_ecs::prelude::*;
use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;

pub type ActionList = Vec<Box<dyn Action>>;
pub type ActionTree = Tree<ActionList>;

#[derive(Default, Deref, DerefMut)]
pub struct ActionGraph(pub DiGraph<ActionList, ()>);


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


	pub fn spawn(
		self,
		world: &mut World,
		target: Entity,
	) -> DiGraph<Entity, ()> {
		let entity_graph = map_graph(self.0, |actions| {
			let mut entity =
				world.spawn((TargetEntity(target), RunTimer::default()));

			for action in actions.iter() {
				action.spawn(&mut entity);
			}
			entity.id()
		});


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
		entity_graph
	}
}


// TODO edge map fn
pub fn map_graph<TypeA, TypeB, F>(
	graph_a: DiGraph<TypeA, ()>,
	mut map_fn: F,
) -> DiGraph<TypeB, ()>
where
	F: FnMut(&TypeA) -> TypeB,
{
	let mut graph_b = DiGraph::<TypeB, ()>::new();

	let nodes: Vec<NodeIndex> = graph_a
		.node_indices()
		.map(|node_index| {
			let node_a = &graph_a[node_index];
			graph_b.add_node(map_fn(node_a))
		})
		.collect();

	// Map original edges to new edges
	for edge in graph_a.edge_indices() {
		let (source, target) = graph_a.edge_endpoints(edge).unwrap();
		let source_index = nodes[source.index()];
		let target_index = nodes[target.index()];
		graph_b.add_edge(source_index, target_index, ());
	}

	graph_b
}
