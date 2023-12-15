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
	let input = &syn::parse::<ItemStruct>(item)?;
	// let input = &input;
	let args = &attributes_map(attr.into(), Some(&["system"]))?;

	let node_struct = parse_node_struct(input, args)?;
	let bevy_message_listener = bevy_message_listener(input);

	// let props = parse_props(&input);
	Ok(quote! {

		use gamai::prelude::*;
		use gamai::exports::*;

		#input
		// #props
		#node_struct

		#bevy_message_listener
	})
}


fn bevy_message_listener(input: &ItemStruct) -> TokenStream {
	let ident = &input.ident;

	let prop_changed = input
		.fields
		.iter()
		.map(|field| {
			let field_ident = &field.ident;
			let ty = &field.ty;
			quote!(Box::new(move |app,value|{
					let mut entity = app.world.entity_mut(entity);
					let mut node = entity.get_mut::<#ident>().unwrap();

					let value = serde_json::from_str::<#ty>(&value)?;
					node.#field_ident = value;
					Ok(())
				}),
			)
		})
		.collect::<TokenStream>();


	quote! {
		impl BevyMessageListener for #ident {
			fn get_listeners(&self, entity: Entity) -> Vec<SetBevyProp>{
				vec![#prop_changed]
			}
		}
	}
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
