use bevy_app::App;
use gamai::prelude::*;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let mut app = App::new();
	let mut message_service = BevyMessageNode::new();
	let entity = app.world.spawn_empty().id();

	message_service.add_prop_listener_raw::<Score>(entity);

	message_service.apply_messages(
		&mut app,
		vec![BuiltinMessage::SetProp(SetProp {
			index: 0,
			value: serde_json::to_string(&Score::Pass)?,
		})],
	)?;

	// app.world.entity_mut(entity).get_mut()

	expect(&app).component(entity)?.to_be(&Score::Pass)?;


	Ok(())
}
