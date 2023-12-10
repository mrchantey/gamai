use bevy_app::App;
// use bevy_ecs::prelude::*;
use gamai::prelude::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	// expect(true).to_be_false()?;


	let mut app = App::new();

	let target = app.world.spawn_empty().id();

	let node = SuccessAction::default().into_node();
	node.add_systems(&mut app);

	let root = node.spawn(&mut app.world, target);

	expect(&app).to_have_component::<Running>(root)?;
	// add `RunResult`, remove `Running`
	app.update();
	expect(&app).not().to_have_component::<Running>(root)?;
	expect(&app).to_have_component::<RunResult>(root)?;
	// remove `Running`
	app.update();
	// remove `RunResult`
	expect(&app).not().to_have_component::<Running>(root)?;
	expect(&app).not().to_have_component::<RunResult>(root)?;


	Ok(())
}
