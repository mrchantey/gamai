use anyhow::Result;
use bevy_ecs::prelude::*;
use gamai::node::ComponentGraph;
use gamai::prelude::*;
use sweet::*;

pub fn assert_nodes<T: Component>(
	entity: Entity,
	world: &World,
	vals: Vec<(usize, bool)>,
) -> Result<()> {
	for (i, v) in vals.iter() {
		expect(ComponentGraph::<Running>::index(entity, world, *i))
			.to_be_option(*v)?;
	}
	Ok(())
}
