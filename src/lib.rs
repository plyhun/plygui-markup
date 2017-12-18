extern crate plygui_api;

extern crate serde;
extern crate serde_json;

use plygui_api::*;

pub use plygui_api::markup;

use std::sync::{Once, ONCE_INIT};

pub enum MarkupError {
	MemberNotFound,
	MemberAlreadyRegistered,
}

static mut IDS: Option<markup::MarkupIds> = None;
static mut REGISTRY: Option<markup::MarkupRegistry> = None;
static INIT: Once = ONCE_INIT;

fn init_registry() {
	unsafe { 
		REGISTRY = Some(Default::default()); 
		IDS = Some(Default::default());
	}
}

pub fn registry<'a>() -> &'a markup::MarkupRegistry {
	INIT.call_once(init_registry);
	unsafe { REGISTRY.as_ref().unwrap() }
}
pub fn registry_mut<'a>() -> &'a mut markup::MarkupRegistry {
	INIT.call_once(init_registry);
	unsafe { REGISTRY.as_mut().unwrap() }
}

pub fn register_member(member_id: &str, member_spawner: fn() -> Box<plygui_api::traits::UiControl>) -> Result<(), MarkupError> {
	INIT.call_once(init_registry);
	unsafe { 
		let registry = REGISTRY.as_mut().unwrap();
		if registry.get(member_id).is_none() {
			registry.insert(member_id.into(), member_spawner); 
			Ok(())
		} else {
			Err(MarkupError::MemberAlreadyRegistered)
		}
	}
}

pub fn unregister_member(member_id: &str) -> Result<(), MarkupError> {
	INIT.call_once(init_registry);
	unsafe { 
		let registry = REGISTRY.as_mut().unwrap();
		if registry.remove(member_id).is_none() {
			Err(MarkupError::MemberNotFound)
		} else {
			Ok(())
		}
	}
}

pub fn parse_markup(json: &str) -> Box<traits::UiControl> {
	INIT.call_once(init_registry);
	
	let markup: markup::Markup = serde_json::from_str(json).unwrap();
	
	let registry = unsafe { REGISTRY.as_ref().unwrap() };
	let mut control = registry.get(&markup.member_type).unwrap()();
	control.fill_from_markup(&markup, registry, unsafe { IDS.as_mut().unwrap() } );
	control
}
