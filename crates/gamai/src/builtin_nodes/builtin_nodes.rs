use crate::builtin_nodes::actions::*;
use crate::builtin_nodes::selectors::*;
use crate::node_collection;


node_collection!(
	BuiltinActions,
	[PassScorer, FailScorer, SuccessAction, FailureAction]
);

node_collection!(
	BuiltinSelectors,
	[FallbackSelector, SequenceSelector, UtilitySelector]
);

node_collection!(BuiltinNodes, [BuiltinActions, BuiltinSelectors]);
