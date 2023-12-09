use bevy_app::App;
use gamai::actions::AlwaysRunResultSuccess;
use gamai::node::IntoNode;
use gamai::node::NodeGraph;
use gamai::prelude::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let mut app = App::new();
	let target = app.world.spawn_empty().id();

	let node = SequenceSelector
		.into_node()
		.with_children((AlwaysRunResultSuccess, AlwaysRunResultSuccess));

	node.add_systems(&mut app);
	let node_entity = node.spawn_running(&mut app.world, target);

	for _ in 0..20 {
		app.update();
		let graph = NodeGraph::<Running>::new(node_entity, &app.world);
		println!("");
		graph.print_tree();
	}

	Ok(())
}
