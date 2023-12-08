use bevy_app::App;
use bevy_ecs::prelude::*;
// use gamai::node::Node;
use gamai::node::{
	IntoNode,
	TargetEntity,
};
use gamai::prelude::*;
use sweet::*;


#[derive(Clone, Component)]
#[node(system=foo)]
struct Foo {
	pub score: Score,
}

impl Default for Foo {
	fn default() -> Self {
		Self {
			score: Score::default(),
		}
	}
}

fn foo() {}

#[sweet_test]
pub fn works() -> Result<()> {
	let mut app = App::new();
	let target = app.world.spawn_empty().id();

	let node = Foo::default().into_node();
	node.add_systems(&mut app);
	let node_entity = node.spawn_graph(&mut app.world, target);
	expect(&app)
		.component(node_entity)?
		.to_be(&TargetEntity(target))?;

	Ok(())
}


#[sweet_test]
pub fn sync_system() -> Result<()> {
	let mut app = App::new();
	let target = app.world.spawn_empty().id();

	let node = Foo { score: Score::Fail }.into_node();
	node.add_systems(&mut app);
	let root = node.spawn_graph(&mut app.world, target);
	app.world
		.entity_mut(root)
		.insert(Foo { score: Score::Pass });

	app.update();

	expect(&app).component(root)?.to_be(&Score::Pass)?;

	Ok(())
}
