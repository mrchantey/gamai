use bevy_app::App;
use gamai::actions::ScoreAlwaysFail;
use gamai::actions::ScoreAlwaysPass;
use gamai::node::IntoNode;
use gamai::node::NodeGraph;
use gamai::prelude::*;
use sweet::*;


#[sweet_test]
pub fn works() -> Result<()> {
	let mut app = App::new();
	let target = app.world.spawn_empty().id();

	let node = UtilitySelector.into_node().with_children((
		ScoreAlwaysFail::default(),
		ScoreAlwaysPass::default(),
	));

	node.add_systems(&mut app);
	let node_entity = node.spawn_running(&mut app.world, target);

	let graph = NodeGraph::<Score>::new(node_entity, &app.world);
	println!("{:?}", graph);

	// select child
	app.update();
	// run child
	// app.update();
	Ok(())
}
