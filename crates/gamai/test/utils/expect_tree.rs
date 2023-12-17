use anyhow::Result;
use bevy_app::App;
use bevy_ecs::prelude::*;
use gamai::prelude::*;
use sweet::*;


pub fn expect_tree<T>(
	app: &mut App,
	entity_graph: &EntityGraph,
	expected: Tree<Option<&T>>,
) -> Result<()>
where
	T: Component + PartialEq + std::fmt::Debug,
{
	let running_tree = ComponentGraph::<T>::new(&app.world, &entity_graph)
		.clone()
		.into_tree();
	expect(running_tree).to_be(expected)
}
