use super::parse_node_struct;
use crate::utils::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;
use syn::Result;

pub fn parse_node(
	attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> Result<TokenStream> {
	let input = syn::parse::<ItemStruct>(item)?;
	let args = attributes_map(attr.into(), Some(&["system"]))?;

	let node_struct = parse_node_struct(&input, &args)?;

	// let props = parse_props(&input);
	Ok(quote! {

		use gamai::prelude::*;
		use gamai::exports::*;

		#input
		// #props
		#node_struct
	})
}


// fn parse_props(input: &ItemStruct) -> TokenStream {
// 	let ident = &input.ident;
// 	input
// 		.fields
// 		.iter()
// 		.map(|field| {
// 			let field_name = &field.ident;
// 			let field_type = &field.ty;
// 			quote! {
// 				impl Prop<#field_type> for #ident {
// 					fn get(&self) -> &#field_type {
// 						&self.#field_name
// 					}

// 					fn set(&mut self, value: #field_type) {
// 						self.#field_name = value;
// 					}
// 				}
// 			}
// 		})
// 		.collect::<TokenStream>()
// }
