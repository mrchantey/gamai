use crate::prelude::*;
use anyhow::anyhow;
use anyhow::Result;
use bevy_app::App;
use bevy_ecs::prelude::*;
use serde::de::DeserializeOwned;


// fn foo(query:Query<Entity,Or Changed<Foo>>)

pub type SetBevyProp = Box<dyn Fn(&mut App, String) -> Result<()>>;


pub trait BevyMessageListener {
	fn get_listeners(&self, entity: Entity) -> Vec<SetBevyProp>;
}


pub type BevyMessageNode = ArrayGraph<SetBevyProp>;

impl BevyMessageNode {
	// pub fn from_node(node: Node, tree: Tree<Entity>) -> Self {
	// 	let this = Self::new();
	// 	// this.items = node.items.into_iter().map(|x| x).collect();
	// 	this
	// }


	pub fn add_prop_listener(&mut self, listener: SetBevyProp) -> usize {
		self.items.push(listener);
		self.items.len() - 1
	}

	pub fn add_prop_listener_raw<T: Component + DeserializeOwned>(
		&mut self,
		entity: Entity,
	) -> usize {
		let listener = Box::new(move |app: &mut App, value: String| {
			let value = serde_json::from_str::<T>(&value)?;
			app.world
				.get_entity_mut(entity)
				.ok_or(anyhow!("Entity does not exist"))?
				.insert(value);
			Ok(())
		});
		self.items.push(listener);
		self.items.len() - 1
	}

	pub fn apply_messages(
		&self,
		app: &mut App,
		messages: impl IntoIterator<Item = BuiltinMessage>,
	) -> Result<()> {
		for message in messages {
			match message {
				BuiltinMessage::SetUpdateSpeed(_) => todo!(),
				BuiltinMessage::LoadTree(_) => todo!(),
				BuiltinMessage::SetProp(prop) => {
					let func = self
						.items
						.get(prop.index)
						.ok_or(anyhow!("Prop index out of bounds"))?;
					func(app, prop.value)?;
				}
			}
		}
		Ok(())
	}
}
