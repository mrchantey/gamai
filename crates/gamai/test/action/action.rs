use super::*;
use bevy_app::App;
use gamai::prelude::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {

	let mut app = App::new();
	let target = app.world.spawn_empty().id();
	let actions = my_action_single_parent();
	let entities = actions.spawn(&mut app.world, target);
	let index = entities.0.node_indices().next().unwrap();

	let message =
		SetActionMessage::new(index, Box::new(MyAction::new(Score::Pass)));

	entities.set_action(&mut app.world, &message)?;

	let entity = entities.node_weight(index).unwrap();

	expect(&app)
		.component::<MyAction>(*entity)?
		.map(|my_entity| my_entity.score)
		.to_be(Score::Pass)?;

	Ok(())
}
