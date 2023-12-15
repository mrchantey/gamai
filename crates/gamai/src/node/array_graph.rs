use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;


pub struct Tree<T> {
	pub value: T,
	pub children: Vec<Tree<T>>,
}


impl<T> Into<Tree<T>> for (T, Vec<Tree<T>>) {
	fn into(self) -> Tree<T> { Tree::<T>::new_with_children(self.0, self.1) }
}


impl<T> Tree<T> {
	pub fn new(value: T) -> Self {
		Self {
			value,
			children: Vec::new(),
		}
	}
	pub fn new_with_children(value: T, children: Vec<Tree<T>>) -> Self {
		Self { value, children }
	}
}

pub struct ArrayGraph<T> {
	pub items: Vec<T>,
	pub children: Vec<ArrayGraph<T>>,
}

impl<T> ArrayGraph<T> {
	pub fn new() -> Self {
		Self {
			items: Vec::new(),
			children: Vec::new(),
		}
	}

	pub fn flatten(self) -> Vec<T> {
		let min_items = self.items.len() + self.children.len();
		let mut items = Vec::with_capacity(min_items);

		for item in self.items.into_iter() {
			items.push(item);
		}
		for child in self.children.into_iter() {
			items.extend(child.flatten());
		}
		items
	}
}
// TODO edge map fn
pub fn map_graph<TypeA, TypeB, F>(
	graph_a: &DiGraph<TypeA, ()>,
	mut map_fn: F,
) -> DiGraph<TypeB, ()>
where
	F: FnMut(NodeIndex, &TypeA) -> TypeB,
{
	let mut graph_out = DiGraph::<TypeB, ()>::new();

	// Map nodes
	let nodes: Vec<NodeIndex> = graph_a
		.node_indices()
		.map(|node_index| {
			let node_a = &graph_a[node_index];
			graph_out.add_node(map_fn(node_index, node_a))
		})
		.collect();

	// Map edges
	for edge in graph_a.edge_indices() {
		let (source, target) = graph_a.edge_endpoints(edge).unwrap();
		let source_index = nodes[source.index()];
		let target_index = nodes[target.index()];
		graph_out.add_edge(source_index, target_index, ());
	}

	graph_out
}
// pub fn map_graphs<TypeA, TypeB, F>(
// 	graphs: Vec<&DiGraph<TypeA, ()>>,
// 	mut map_fn: F,
// ) -> DiGraph<TypeB, ()>
// where
// 	F: FnMut(&TypeA) -> TypeB,
// {
// 	let mut graph_out = DiGraph::<TypeB, ()>::new();

// 	// Map nodes
// 	let nodes: Vec<NodeIndex> = graphs[0]
// 		.node_indices()
// 		.map(|node_index| {
// 			let node_a = &graph_a[node_index];
// 			graph_out.add_node(map_fn(node_a))
// 		})
// 		.collect();

// 	// Map edges
// 	for edge in graph_a.edge_indices() {
// 		let (source, target) = graph_a.edge_endpoints(edge).unwrap();
// 		let source_index = nodes[source.index()];
// 		let target_index = nodes[target.index()];
// 		graph_out.add_edge(source_index, target_index, ());
// 	}

// 	graph_out
// }
