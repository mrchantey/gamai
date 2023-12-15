use crate::utils::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use syn::Expr;
use syn::ItemStruct;
use syn::Result;


pub fn parse_action(
	attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> Result<TokenStream> {
	let input = &syn::parse::<ItemStruct>(item)?;
	let args = &attributes_map(attr.into(), Some(&["system"]))?;

	let action_trait = action_trait(input, args);
	let into = into(input);

	Ok(quote! {
		use gamai::prelude::*;
		use gamai::exports::*;
		#input
		#action_trait
		#into
	})
}

fn into(input: &ItemStruct) -> TokenStream {
	let _ident = &input.ident;
	quote! {
		// impl Into<Box<dyn Action>> for #ident {
		// 	fn into(self) -> Box<dyn Action> {
		// 		Box::new(self)
		// 	}
		// }
		// impl Into<ActionTree> for #ident {
		// 	fn into(self) -> ActionTree {
		// 		ActionTree::new(vec![Box::new(self)])
		// 	}
		// }
	}
}


fn action_trait(
	input: &ItemStruct,
	args: &HashMap<String, Option<Expr>>,
) -> TokenStream {
	let ident = &input.ident;

	let meta = meta(input);
	let spawn = spawn(input);
	let tick_system = tick_system(args);
	let post_tick_system = post_tick_system(input);
	let prop_listeners = prop_listeners(input);

	quote! {
		#[typetag::serde]
		impl Action for #ident {
			fn duplicate(&self) -> Box<dyn Action> {
				Box::new(self.clone())
			}
			#meta

			#spawn

			#tick_system
			#post_tick_system
			#prop_listeners

		}
	}
}

static ACTION_ID: AtomicUsize = AtomicUsize::new(0);


fn meta(input: &ItemStruct) -> TokenStream {
	let ident = &input.ident;
	let name = ident.to_string();
	let action_id = ACTION_ID.fetch_add(1, Ordering::SeqCst);

	quote! {
		fn meta(&self) -> ActionMeta {
			ActionMeta {
				id: #action_id,
				name: #name
			}
		}
	}
}

fn tick_system(args: &HashMap<String, Option<Expr>>) -> TokenStream {
	let expr = args.get("system").unwrap().as_ref().unwrap();
	quote! {
		fn tick_system(&self) -> SystemConfigs {
			#expr.into_configs()
		}
	}
}

fn post_tick_system(input: &ItemStruct) -> TokenStream {
	let ident = &input.ident;

	let prop_types = input
		.fields
		.iter()
		.map(|field| {
			let ty = &field.ty;
			quote!(&mut #ty)
		})
		.collect::<TokenStream>();

	let prop_destructs = input
		.fields
		.iter()
		.map(|field| {
			let field_ident = &field.ident;
			quote!(mut #field_ident, )
		})
		.collect::<TokenStream>();

	let prop_assignments = input
		.fields
		.iter()
		.map(|field| {
			let field_ident = &field.ident;
			quote!(*#field_ident = value.#field_ident;)
		})
		.collect::<TokenStream>();

	quote! {
		fn post_tick_system(&self) -> SystemConfigs {

			fn post_sync_system(mut query: Query<(&#ident,#prop_types), Changed<#ident>>){
				for (value, #prop_destructs) in query.iter_mut(){
					#prop_assignments
				}
			}

			post_sync_system.into_configs()
		}
	}
}

fn spawn(input: &ItemStruct) -> TokenStream {
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
		fn spawn(&self, entity: &mut EntityWorldMut<'_>) {
			entity.insert(self.clone());
			#insert_props
		}
		fn spawn_with_command(&self, entity: &mut EntityCommands) {
			entity.insert(self.clone());
			#insert_props
		}
	}
}

fn prop_listeners(input: &ItemStruct) -> TokenStream {
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
		fn prop_listeners(&self, entity: Entity) -> Vec<SetBevyProp>{
			vec![#prop_changed]
		}
	}
}
