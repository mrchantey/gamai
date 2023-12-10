use crate::prelude::*;
use bevy_ecs::all_tuples;
pub trait IntoNode {
	fn into_node(self) -> Node;
}
// impl IntoNode for () {
// 	fn into_node(self) -> Node {
// 		Node {
// 			items: Vec::new(),
// 			children: Vec::new(),
// 		}
// 	}
// }

impl IntoNode for Node {
	fn into_node(self) -> Node { self }
}

macro_rules! tuples_into_node {
	($($name: ident),*) => {
		impl<$($name: IntoNode),*> IntoNode for ($($name,)*) {
			fn into_node(self) -> Node {
				#[allow(unused_mut)]
				let mut this = Node {
					items: Vec::new(),
					children: Vec::new(),
				};

				#[allow(non_snake_case)]
				let ($($name,)*) = self;
				#[allow(non_snake_case)]
				{
					$(let $name = $name.into_node();)*
					$(this.items.extend($name.items);)*
					$(this.children.extend($name.children);)*
				}
				this
			}
		}
	}
}
all_tuples!(tuples_into_node, 0, 15, T);

impl<T> IntoNode for T
where
	T: NodeStruct,
{
	fn into_node(self) -> Node {
		Node {
			items: vec![Box::new(self)],
			children: Vec::new(),
		}
	}
}

pub trait IntoNodes {
	fn into_nodes(self) -> Vec<Node>;
}

impl IntoNodes for Node {
	fn into_nodes(self) -> Vec<Node> { vec![self] }
}

macro_rules! tuples_into_nodes {
	($($name: ident),*) => {
		impl<$($name: IntoNode),*> IntoNodes for ($($name,)*) {
			fn into_nodes(self) -> Vec<Node> {
				#[allow(non_snake_case)]
				let ($($name,)*) = self;
				vec![$($name.into_node()),*]
			}
		}
	}
}
all_tuples!(tuples_into_nodes, 0, 15, T);
