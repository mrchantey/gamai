use anyhow::Result;
use bevy_ecs::prelude::*;
use gamai::node::NodeGraph;
use gamai::prelude::*;
use sweet::*;

pub fn assert_nodes<T: Component>(
	entity: Entity,
	world: &World,
	vals: Vec<(usize, bool)>,
) -> Result<()> {
	for (i, v) in vals.iter() {
		expect(NodeGraph::<Running>::index(entity, world, *i))
			.to_be_option(*v)?;
	}
	Ok(())
}
