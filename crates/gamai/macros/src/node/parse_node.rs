use crate::utils::*;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::Expr;
use syn::Ident;
use syn::ItemStruct;
use syn::Result;

pub fn parse_node(
	attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> Result<TokenStream> {
	let input = syn::parse::<ItemStruct>(item)?;
	let args = attributes_map(attr.into(), Some(&["system"]))?;

	let node_system = parse_node_system(&input.ident, &args)?;
	let node_struct = parse_node_struct(&input);

	let props = parse_props(&input);
	let sync_system = parse_sync_system(&input);


	Ok(quote! {

		use gamai::prelude::*;
		use gamai::exports::*;

		#input
		#props
		#sync_system
		#node_system
		#node_struct
	})
}

fn parse_sync_system(input: &ItemStruct) -> TokenStream {
	let ident = &input.ident;

	let query_field_types = input
		.fields
		.iter()
		.map(|field| {
			let ty = &field.ty;
			quote!(&mut #ty)
		})
		.collect::<TokenStream>();

	let query_field_destructs = input
		.fields
		.iter()
		.map(|field| {
			let field_ident = &field.ident;
			quote!(mut #field_ident, )
		})
		.collect::<TokenStream>();

	let query_field_assignments = input
		.fields
		.iter()
		.map(|field| {
			let field_ident = &field.ident;
			quote!(*#field_ident = value.#field_ident;)
		})
		.collect::<TokenStream>();

	quote! {
		//TODO query includes props

		impl SyncSystem for #ident{
			fn get_sync_system(&self) -> SystemConfigs {
				#[allow(non_snake_case)]
				fn sync_system(mut query: Query<(&#ident,#query_field_types), Changed<#ident>>){

					for (value, #query_field_destructs) in query.iter_mut(){
						#query_field_assignments
					}
				}

				sync_system.into_configs()
			}
		}

	}
}


fn parse_node_struct(input: &ItemStruct) -> TokenStream {
	let ident = &input.ident;

	let insert_props = input
		.fields
		.iter()
		.map(|field| {
			let field_name = &field.ident;
			// let field_type = &field.ty;
			quote! {
				entity.insert(self.#field_name.clone());
			}
		})
		.collect::<TokenStream>();

	quote! {
		impl NodeStruct for #ident{
			fn init(&self, entity: &mut EntityWorldMut<'_>) {
				entity.insert(self.clone());
				#insert_props
			}
			fn init_from_command(&self, entity: &mut EntityCommands) {
				entity.insert(self.clone());
				#insert_props
			}
		}
	}
}

fn parse_props(input: &ItemStruct) -> TokenStream {
	let ident = &input.ident;
	input
		.fields
		.iter()
		.map(|field| {
			let field_name = &field.ident;
			let field_type = &field.ty;
			quote! {
				impl Prop<#field_type> for #ident {
					fn get(&self) -> &#field_type {
						&self.#field_name
					}

					fn set(&mut self, value: #field_type) {
						self.#field_name = value;
					}
				}
			}
		})
		.collect::<TokenStream>()
}


fn parse_node_system(
	ident: &Ident,
	args: &HashMap<String, Option<Expr>>,
) -> Result<TokenStream> {
	let system_ident = args
		.get("system")
		.ok_or_else(|| {
			syn::Error::new(
				Span::call_site(),
				"Expected `system = \"path::to::system\"`",
			)
		})?
		.as_ref()
		.ok_or_else(|| {
			syn::Error::new(
				Span::call_site(),
				"Expected `system = \"path::to::system\"`",
			)
		})?;

	Ok(quote! {
		impl NodeSystem for #ident {
			fn get_node_system(&self) -> SystemConfigs {
				#system_ident.into_configs()
			}
		}

	})
}
