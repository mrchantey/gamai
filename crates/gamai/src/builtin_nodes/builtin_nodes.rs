use crate::builtin_nodes::actions::*;
use crate::builtin_nodes::selectors::*;
use crate::node_list;


node_list!(
	BuiltinActions,
	[PassScorer, FailScorer, SuccessAction, FailureAction]
);

node_list!(
	BuiltinSelectors,
	[FallbackSelector, SequenceSelector, UtilitySelector]
);

node_list!(BuiltinNodes, [BuiltinActions, BuiltinSelectors]);
