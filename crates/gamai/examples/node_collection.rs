use gamai::node_list;
use gamai::prelude::*;
pub fn main() {}


node_list!(
	FoobarList,
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
