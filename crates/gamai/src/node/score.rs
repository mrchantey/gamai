use bevy_ecs::prelude::*;
use std::cmp::Ordering;
use std::fmt::Debug;


/// Used to indicate to selectors how favorable a child node would be to run.
#[derive(Default, Debug, Clone, Copy, Component, PartialEq)]
pub enum Score {
	#[default]
	/// The node should not run.
	Fail,
	/// The node has a `0..1` weight where 1 is most favourable.
	Weight(f32),
	/// The node should run.
	Pass,
}


impl PartialOrd for Score {
	#[allow(unused_variables, unreachable_code)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		todo!("test this");
		let val = match (self, other) {
			(Score::Fail, Score::Fail) => Ordering::Equal,
			(Score::Fail, _) => Ordering::Less,
			(_, Score::Fail) => Ordering::Greater,
			(Score::Pass, Score::Pass) => Ordering::Equal,
			(Score::Pass, _) => Ordering::Less,
			(_, Score::Pass) => Ordering::Greater,
			(Score::Weight(w1), Score::Weight(w2)) => w1.total_cmp(&w2),
		};
		Some(val)
	}
}
