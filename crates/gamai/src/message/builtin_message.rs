use crate::prelude::*;
use anyhow::Result;

pub enum UpdateSpeed {
	Playing,
	PlayingAtSpeed(f32),
	Paused,
}


pub struct SetProp {
	pub index: usize,
	pub value: String,
}


pub enum BuiltinMessage {
	SetUpdateSpeed(UpdateSpeed),
	LoadTree(TypedNode<BuiltinNodes>),
	SetProp(SetProp),
}


pub trait MessageReader<T> {
	fn read_messages(&mut self, messages: Vec<T>) -> Result<()>;
}
pub trait MessageWriter<T> {
	fn write_messages(&mut self) -> Result<Vec<T>>;
}
