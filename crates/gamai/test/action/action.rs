use bevy_app::App;
use bevy_ecs::prelude::*;
use gamai::prelude::*;
use sweet::*;


#[action(system=my_action)]
#[derive(Default, Clone, Component, Serialize, Deserialize)]
pub struct MyAction {
	pub score: Score,
}
impl MyAction {
	pub fn new(score: Score) -> Self { Self { score } }
}

fn my_action() {}


#[sweet_test]
pub fn works() -> Result<()> {
	let mut app = App::new();

	let target = app.world.spawn_empty().id();

	let actions = ActionTree::from_action(MyAction::default())
		.with_child(ActionTree::from_action(MyAction::default()))
		.into_graph();


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
