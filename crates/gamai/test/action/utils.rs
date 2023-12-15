use bevy_ecs::prelude::*;
use gamai::prelude::*;


#[action(system=my_action)]
#[derive(Default, Clone, Component, Serialize, Deserialize)]
pub struct MyAction {
	pub score: Score,
}
impl MyAction {
	pub fn new(score: Score) -> Self { Self { score } }
}

fn my_action() {}


pub fn my_action_single_parent() -> ActionGraph {
	ActionTree::from_action(MyAction::default())
		.with_child(ActionTree::from_action(MyAction::default()))
		.into_graph()
}
