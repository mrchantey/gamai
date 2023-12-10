use gamai::node_collection;
use gamai::prelude::*;
pub fn main() {}


node_collection!(
	FoobarCollection,
	[
		PassScorer,
		FailScorer,
		SuccessAction,
		FailureAction,
		FallbackSelector,
		SequenceSelector,
		UtilitySelector
	]
);
