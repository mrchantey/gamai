use crate::prelude::*;
use serde::Deserialize;
use serde::Serialize;

/// A version of graphs that can be serialized and deserialized.
#[derive(Serialize, Deserialize)]
pub struct TypedNode<T: NodeStruct> {
	pub items: Vec<T>,
	pub children: Vec<TypedNode<T>>,
}

impl<T: NodeStruct> TypedNode<T> {
	pub fn new(items: Vec<T>, children: Vec<TypedNode<T>>) -> Self {
		Self { items, children }
	}
}

impl<T: NodeStruct> IntoNode for TypedNode<T> {
	fn into_node(self) -> Node {
		Node {
			items: self
				.items
				.into_iter()
				.map(|x| Box::new(x) as Box<dyn NodeStruct>)
				.collect(),
			children: self
				.children
				.into_iter()
				.map(|x| x.into_node())
				.collect(),
		}
	}
}
