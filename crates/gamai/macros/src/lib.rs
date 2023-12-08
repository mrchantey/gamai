mod node;
use node::*;
use proc_macro::TokenStream;
mod utils;
// pub(crate) use utils::*;

/// Annotate a struct as a node, defining its corresponding system.
///
/// A node treats each field as a [`Prop`] which is a supertrait of [`Component, Clone`]
/// and only one of each type is allowed. This pattern allows all node systems to be run in parallel
/// and their props to be efficiently synced later.
///
/// ```rust
///
/// // a similar thing can be done for `AlwaysSuccceed`
///
/// #[node(always_pass)]
/// struct AlwaysPass{
///   score: Score,
/// }
///
///
/// fn always_pass(entities: Query<&mut AlwaysPass, With<Running>>) {
///
/// 	for mut node in entities.iter_mut() {
///   	node.score = Score::Pass;
/// 	}
/// }
///
/// ```
///
/// It also adds a syncing system
/// TODO use Added, Removed instead of Option<AlwaysPass>
/// ```rust
/// fn sync_always_pass(mut query: Query<(Option<&mut Score>, Option<AlwaysPass>), With<AlwaysPass>>) {
///
///  // if they are equal, use commands or mutate etc.
///
/// }
/// ```
///
#[proc_macro_attribute]
pub fn node(attr: TokenStream, item: TokenStream) -> TokenStream {
	parse_node(attr, item)
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}


/// Used for selectors aka non-leaf nodes.
/// Define props required for each child. Children should only be added to this
/// node if they have all the required props.
#[proc_macro_attribute]
pub fn child_props(_attr: TokenStream, _item: TokenStream) -> TokenStream {
	todo!()
}


/// Assign a system to a node.
///
/// ```rust
/// #[node_system(my_action)]
/// struct MyAction;
///
/// fn my_action(query: Query<&mut MyAction, With<Running>>) {
///   for action in query.iter_mut() {
/// 		// ...
/// 	}
/// }
///
/// ```
///
///
#[proc_macro_attribute]
pub fn node_system(attr: TokenStream, item: TokenStream) -> TokenStream {
	parse_node_system(attr, item)
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}
