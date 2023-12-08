use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;
use syn::Result;

pub fn parse_node_system(
	_attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> Result<TokenStream> {
	let input = syn::parse::<ItemStruct>(item)?;

	// let prop_impls = parse_props(&input);
	// let sync_system = parse_sync_system(&input);

	Ok(quote! {

		use gamai::prelude::*;
		use gamai::exports::*;

		#input
		// #prop_impls
		// #sync_system
	})
}
