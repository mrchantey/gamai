// use bevy_app::App;
use gamai::prelude::*;
use sweet::*;



#[node(system=foo)]
#[derive(Clone, Component)]
struct Foo {}

fn foo() {}

#[sweet_test]
pub fn works() -> Result<()> {
	// let mut app = App::new();

	// let foo = app.world.spawn_empty();

	Ok(())
}
