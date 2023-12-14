use bevy_app::App;
use gamai::message::BevyMessageService;
use gamai::message::BuiltinMessage;
use gamai::node::Score;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let mut app = App::new();
	let mut message_service = BevyMessageService::new();
	let entity = app.world.spawn_empty().id();

	message_service.add_prop_listener::<Score>(entity);

	message_service.apply_messages(
		&mut app,
		vec![BuiltinMessage::SetProp(gamai::message::SetProp {
			index: 0,
			value: serde_json::to_string(&Score::Pass)?,
		})],
	)?;

	expect(&app).component(entity)?.to_be(&Score::Pass)?;


	Ok(())
}
