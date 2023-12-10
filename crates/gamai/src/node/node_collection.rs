/// Define a node collection
///
/// ```rust
///
/// node_collection!(BuiltinNodes,[
/// 	Run,
/// 	Hide,
/// 	ChooseWhatToDo
/// ]);
/// ```
///
#[macro_export]
macro_rules! node_collection {
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

		impl gamai::prelude::IntoNodeStruct for $name {
			fn into_node_struct(&self) -> &dyn NodeStruct {
				match self {
					$(Self::$variant(x) => x,)*
				}
			}
		}
	};
}
