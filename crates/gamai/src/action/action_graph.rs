use crate::prelude::*;
use bevy_app::App;
use bevy_app::Update;
use bevy_derive::Deref;
use bevy_derive::DerefMut;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;
use serde::Deserialize;
use serde::Serialize;

pub type ActionList = Vec<Box<dyn Action>>;
pub type ActionTree = Tree<ActionList>;

impl Into<ActionTree> for Box<dyn Action> {
	fn into(self) -> ActionTree { ActionTree::new(vec![self]) }
}


impl ActionTree {
	pub fn from_action(value: impl Action) -> Self {
		Self {
			value: vec![Box::new(value)],
			children: vec![],
		}
	}
	pub fn into_graph(self) -> ActionGraph { ActionGraph::from_tree(self) }
}

#[derive(Default, Deref, DerefMut, Serialize, Deserialize)]
pub struct ActionGraph(pub DiGraph<ActionList, ()>);

impl Clone for ActionGraph {
	fn clone(&self) -> Self {
		let graph = map_graph(&self.0, |_, action_list| {
			action_list
				.into_iter()
				.map(|action| action.duplicate())
				.collect::<Vec<_>>()
		});
		ActionGraph(graph)
	}
}

impl ActionGraph {
	pub fn new() -> Self { Self::default() }

	pub fn from_tree(root: impl Into<ActionTree>) -> Self {
		let mut this = Self::new();
		this.add_recursive(root.into());
		this
	}

	pub fn add_recursive(&mut self, tree: ActionTree) -> NodeIndex {
		let ActionTree { value, children } = tree;
		let node = self.add_node(value);

		for child in children {
			let index = self.add_recursive(child);
			self.add_edge(node, index, ());
		}

		node
	}

	pub fn spawn(&self, world: &mut World, target: Entity) -> EntityGraph {
		// create entities & actions
		let entity_graph = map_graph(&self.0, |_, actions| {
			let mut entity =
				world.spawn((TargetEntity(target), RunTimer::default()));

			for action in actions.iter() {
				action.spawn(&mut entity);
			}
			entity.id()
		});

		// create edges
		for (index, entity) in Iterator::zip(
			entity_graph.node_indices(),
			entity_graph.node_weights(),
		) {
			let children = entity_graph
				.neighbors_directed(index, petgraph::Direction::Outgoing)
				.map(|index| entity_graph[index])
				.collect::<Vec<_>>();
			world.entity_mut(*entity).insert(Edges(children));
		}

		world
			.entity_mut(*entity_graph.root().unwrap())
			.insert(Running);

		EntityGraph(entity_graph)
	}

	pub fn try_add_systems_to_default_schedule(&self, app: &mut App) {
		self.try_add_systems(app, || {
			ActionSchedule::new(Update, PreTickSet, TickSet, PostTickSet)
		})
	}
	pub fn try_add_systems<
		Schedule: ScheduleLabel + Clone,
		PreTickSet: SystemSet + Clone,
		TickSet: SystemSet + Clone,
		PostTickSet: SystemSet + Clone,
	>(
		&self,
		app: &mut App,
		init_tracker: impl Fn() -> ActionSchedule<
			Schedule,
			PreTickSet,
			TickSet,
			PostTickSet,
		>,
	) {
		if false
			== app.world.contains_resource::<ActionSchedule<
				Schedule,
				PreTickSet,
				TickSet,
				PostTickSet,
			>>() {
			let tracker = init_tracker();

			app.configure_sets(
				tracker.schedule.clone(),
				tracker.pre_tick_set.clone(),
			);
			app.configure_sets(
				tracker.schedule.clone(),
				tracker.tick_set.clone().after(tracker.pre_tick_set.clone()),
			);
			app.configure_sets(
				tracker.schedule.clone(),
				tracker
					.post_tick_set
					.clone()
					.after(tracker.pre_tick_set.clone()),
			);

			app.add_systems(
				tracker.schedule.clone(),
				apply_deferred
					.after(tracker.tick_set.clone())
					.before(tracker.post_tick_set.clone()),
			);
			app.add_systems(
				Update,
				sync_running.in_set(tracker.post_tick_set.clone()),
			);

			app.world.insert_resource(tracker);
		}

		for actions in self.node_weights() {
			for action in actions {
				let mut action_schedule = app
					.world
					.get_resource_mut::<ActionSchedule<Schedule, PreTickSet, TickSet, PostTickSet>>(
					)
					.unwrap();

				if action_schedule.try_add_action(action.as_ref()) {
					let schedule = action_schedule.schedule.clone();
					let tick_set = action_schedule.tick_set.clone();
					let post_tick_set = action_schedule.post_tick_set.clone();
					app.add_systems(
						schedule.clone(),
						action.tick_system().in_set(tick_set),
					);
					app.add_systems(
						schedule,
						action.post_tick_system().in_set(post_tick_set),
					);
				}
			}
		}
	}
}
