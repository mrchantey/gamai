use super::*;
use bevy_app::App;
use gamai::prelude::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let mut app = App::new();
	let target = app.world.spawn_empty().id();

	let node = UtilitySelector.into_node().with_children((
		(FailScorer { score: Score::Fail }, SuccessAction),
		(PassScorer::default(), SuccessAction),
	));

	node.add_systems(&mut app);
	let root = node.spawn(&mut app.world, target).value;

	app.update();
	assert_nodes::<Running>(
		root,
		&app.world,
		vec![(0, true), (1, false), (2, true)],
	)?;

	app.update();
	assert_nodes::<Running>(
		root,
		&app.world,
		vec![(0, true), (1, false), (2, false)],
	)?;

	app.update();
	assert_nodes::<Running>(
		root,
		&app.world,
		vec![(0, false), (1, false), (2, false)],
	)?;

	expect(ComponentGraph::<RunResult>::index(root, &app.world, 0))
		.to_be(Some(&RunResult::Success))?;

	Ok(())
}
