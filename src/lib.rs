extern crate plygui_api;

extern crate serde;
extern crate serde_json;

use plygui_api::*;

pub use plygui_api::markup;

use std::collections::HashMap;

pub fn parse_markup<F>(json: &str, constructor: F) -> (Box<traits::UiControl>, markup::MarkupIds) where F: Fn(&markup::Markup) -> Box<traits::UiControl> {
	let mut ids: markup::MarkupIds = HashMap::new();
	let markup: markup::Markup = serde_json::from_str(json).unwrap();
	
	let member = parse_node(&markup, &mut ids, constructor);
	(member, ids)
}

fn parse_node<F>(node: &markup::Markup, ids: &mut markup::MarkupIds, constructor: F) -> Box<traits::UiControl> where F: Fn(&markup::Markup) -> Box<traits::UiControl> {
	constructor(node)
}