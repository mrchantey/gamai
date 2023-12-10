use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::Expr;
use syn::ItemStruct;
use syn::Result;



pub fn parse_node_struct(
	input: &ItemStruct,
	args: &HashMap<String, Option<Expr>>,
) -> Result<TokenStream> {
	let ident = &input.ident;
	let struct_insert = parse_struct_insert(input);
	let sync_system = parse_sync_system(input);
	let node_system = parse_node_system(args)?;

	Ok(quote! {
		impl NodeStruct for #ident{
			#struct_insert
			#sync_system
			#node_system
		}
	})
}
fn parse_struct_insert(input: &ItemStruct) -> TokenStream {
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



fn parse_node_system(
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
		fn get_node_system(&self) -> SystemConfigs {
			#system_ident.into_configs()
		}
	})
}
