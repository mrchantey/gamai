/// Define a node list. This macro accepts a name and a list of node structs.
///
/// ```rust
///
/// node_list!(AgentNodes, [
/// 	Run,
/// 	Hide,
/// 	ChooseWhatToDo
/// ]);
/// ```
///
#[macro_export]
macro_rules! node_list {
	($name:ident, [$($variant:ident),*]) => {
		#[allow(unused_imports)]
		use gamai::prelude::*;
		#[allow(unused_imports)]
		use gamai::exports::*;
		#[derive(Serialize, Deserialize)]
		// #[serde(tag = "type")]
		pub enum $name {
			$($variant($variant),)*
		}

		// impl IntoNodeStruct for $name {
		// 	fn into_node_struct(&self) -> &dyn NodeStruct {
		// 		match self {
		// 			$(Self::$variant(x) => x,)*
		// 		}
		// 	}
		// }

		// impl NodeStructVariants for $name {
		// 	fn get_node_struct_variants() -> Vec<Box<dyn NodeStruct>> {
		// 		let mut vec = Vec::new();
		// 		$(
		// 			vec.extend($variant::get_node_struct_variants());
		// 		)*
		// 		vec
		// 		}
		// }
	};
}