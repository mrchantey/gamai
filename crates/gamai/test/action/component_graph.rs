use super::test_action_graph;
use bevy_app::App;
use gamai::prelude::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let mut app = App::new();
	let target = app.world.spawn_empty().id();
	let actions = test_action_graph();
	let entities = actions.spawn(&mut app.world, target);

	let entity = *entities.node(2).unwrap();
	app.world.entity_mut(entity).insert(Score::Pass);
	let scores = ComponentGraph::<Score>::new(&app.world, &entities);

	expect(scores.node(0)).to_be(Some(&Some(&Score::Fail)))?;
	expect(scores.node(1)).to_be(Some(&Some(&Score::Fail)))?;
	expect(scores.node(2)).to_be(Some(&Some(&Score::Pass)))?;
	expect(scores.node(3)).to_be(Some(&Some(&Score::Fail)))?;

	Ok(())
}
