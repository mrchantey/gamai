use super::*;
use bevy_app::App;
use gamai::prelude::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let mut app = App::new();
	let target = app.world.spawn_empty().id();
	let actions = test_action_graph();
	let entities = actions.spawn(&mut app.world, target);
	let index = entities.0.node_indices().next().unwrap();

	let message =
		SetActionMessage::new(index, Box::new(TestAction::new(Score::Pass)));

	entities.set_action(&mut app.world, &message)?;

	let entity = entities.node_weight(index).unwrap();

	expect(&app)
		.component::<TestAction>(*entity)?
		.map(|my_entity| my_entity.score)
		.to_be(Score::Pass)?;

	Ok(())
}


#[sweet_test]
pub fn default_components() -> Result<()> {
	let mut app = App::new();
	let target = app.world.spawn_empty().id();
	let actions = test_action_graph();
	let entities = actions.spawn(&mut app.world, target);
	let entity = *entities.root().unwrap();

	expect(&app).to_have_component::<TestAction>(entity)?;
	expect(&app).to_have_component::<TargetEntity>(entity)?;
	expect(&app).to_have_component::<RunTimer>(entity)?;
	expect(&app).to_have_component::<Score>(entity)?;


	Ok(())
}



#[sweet_test]
pub fn sync_system() -> Result<()> {
	let mut app = App::new();
	let target = app.world.spawn_empty().id();
	let actions = test_action_graph();
	let entities = actions.spawn(&mut app.world, target);
	let entity = *entities.root().unwrap();

	actions.add_systems(&mut app);
	app.world
		.entity_mut(entity)
		.insert(TestAction::new(Score::Pass));

	expect(&app).component(entity)?.to_be(&Score::Fail)?;
	app.update();
	expect(&app).component(entity)?.to_be(&Score::Pass)?;

	Ok(())
}
