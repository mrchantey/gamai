use crate::prelude::*;
use bevy_app::App;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use bevy_utils::HashSet;
use std::any::Any;
use std::any::TypeId;

#[derive(Resource)]
pub struct ActionSchedule<
	Schedule: ScheduleLabel + Clone,
	PreTickSet: SystemSet + Clone,
	TickSet: SystemSet + Clone,
	PostTickSet: SystemSet + Clone,
> {
	pub added: HashSet<TypeId>,
	pub schedule: Schedule,
	pub pre_tick_set: PreTickSet,
	pub tick_set: TickSet,
	pub post_tick_set: PostTickSet,
}

impl<
		Schedule: ScheduleLabel + Clone,
		PreTickSet: SystemSet + Clone,
		TickSet: SystemSet + Clone,
		PostTickSet: SystemSet + Clone,
	> ActionSchedule<Schedule, PreTickSet, TickSet, PostTickSet>
{
	pub fn new(
		schedule: Schedule,
		pre_tick_set: PreTickSet,
		tick_set: TickSet,
		post_tick_set: PostTickSet,
	) -> Self {
		Self {
			added: HashSet::default(),
			schedule,
			pre_tick_set,
			tick_set,
			post_tick_set,
		}
	}

	pub fn try_add_action(&mut self, action: &dyn Action) -> bool {
		if self.added.contains(&action.type_id()) {
			return false;
		}
		self.added.insert(action.type_id());
		true
	}

	pub fn add_action_systems(&self, app: &mut App, action: &dyn Action) {
		app.add_systems(
			self.schedule.clone(),
			action.tick_system().in_set(self.tick_set.clone()),
		);
		app.add_systems(
			self.schedule.clone(),
			action.post_tick_system().in_set(self.post_tick_set.clone()),
		);
	}
}
