use anyhow::Result;
use bevy_app::App;
use bevy_ecs::prelude::*;
use bevy_utils::HashMap;
use gamai::builtin_nodes::BuiltinNodes;
use gamai::message::BuiltinMessage;
use gamai::node::TypedNode;


type PropSignals = Vec<Box<dyn Fn(String) -> Result<()>>>;

fn main() {
	let app = App::new();

	let mut prop_signals = PropSignals::new();

	prop_signals.push(Box::new(|val: String| Ok(())));
	prop_signals.push(Box::new(|val: String| Ok(())));
}


fn receive_message(message: BuiltinMessage, signals: &PropSignals) {
	match message {
		BuiltinMessage::SetUpdateSpeed(_) => todo!(),
		BuiltinMessage::LoadTree(_) => todo!(),
		BuiltinMessage::SetProp(msg) => signals[msg.index](msg.value),
	}
}

trait ChannelEvent<const CHANNEL: u32> {
	fn channel() -> u32 { CHANNEL }
}

struct EventCaller {
	pub children: Vec<EventCaller>,
}


struct BaseEvent {
	pub channel: u32,
}

struct Base {
	game_events: Vec<GameEvent>,
	page_events: Vec<PageEvent>,
	game_cx: App,
	page_cx: Box<dyn PageCx>,
}


impl Base {
	fn update() {}
}

trait PageCx {
	fn send_event(&self, event: PageEvent);
}

struct GameEvent {}

struct PageEvent {}



// each NodeStruct must also implement UiBuilder
// Node also implements UiBuilder, which adds
struct MyNode;

trait IntoUiProps {
	fn into_props(&self) -> Vec<Box<dyn UiProp>>;
}


struct GamaiEvent {}

struct Event {}

#[derive(Component)]
struct PropHidden;


struct UiNode {
	children: Vec<UiNode>,
	props: Vec<Box<dyn UiProp>>,
}

trait UiProp {
	fn entity(&self) -> Entity;
}

trait UiBuilder {}


impl UiBuilder for MyNode {}
