use bevy_app::App;
// use gamai::builtin_nodes::BuiltinNodes;
// use gamai::node::NodeSerde;
use gamai::prelude::*;
use sweet::*;


#[sweet_test]
pub fn works() -> Result<()> {
	let val = TypedNode::<BuiltinNodes>::new(
		vec![BuiltinNodes::BuiltinSelectors(
			BuiltinSelectors::UtilitySelector(UtilitySelector::default()),
		)],
		vec![
			TypedNode::<BuiltinNodes>::new(
				vec![BuiltinNodes::BuiltinActions(BuiltinActions::FailScorer(
					FailScorer::default(),
				))],
				vec![],
			),
			TypedNode::<BuiltinNodes>::new(
				vec![BuiltinNodes::BuiltinActions(BuiltinActions::PassScorer(
					PassScorer::default(),
				))],
				vec![],
			),
		],
	);

	let str = serde_json::to_string_pretty(&val)?;
	// println!("{}", str);

	let a: TypedNode<BuiltinNodes> = serde_json::from_str(&str)?;
	let b = a.into_node();
	let mut app = App::new();
	let target = app.world.spawn_empty().id();
	let root = b.spawn(&mut app.world, target).value;

	expect(ComponentGraph::<Score>::index(root, &app.world, 0)).to_be_none()?;
	expect(ComponentGraph::<Score>::index(root, &app.world, 1))
		.to_be(Some(&Score::Fail))?;
	expect(ComponentGraph::<Score>::index(root, &app.world, 2))
		.to_be(Some(&Score::Pass))?;

	Ok(())
}
