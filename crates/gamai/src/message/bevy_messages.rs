use crate::prelude::*;
use anyhow::anyhow;
use anyhow::Result;
use bevy_app::App;
use bevy_ecs::prelude::*;
use serde::de::DeserializeOwned;


// fn foo(query:Query<Entity,Or Changed<Foo>>)

type SetProp = Box<dyn Fn(&mut App, String) -> Result<()>>;

pub struct BevyMessageService {
	prop_listeners: Vec<SetProp>,
}

impl BevyMessageService {
	pub fn new() -> Self {
		Self {
			prop_listeners: Vec::new(),
		}
	}

	pub fn add_prop_listener<T: Component + DeserializeOwned>(
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
		self.prop_listeners.push(listener);
		self.prop_listeners.len() - 1
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
						.prop_listeners
						.get(prop.index)
						.ok_or(anyhow!("Prop index out of bounds"))?;
					func(app, prop.value)?;
				}
			}
		}
		Ok(())
	}
}
