extern crate plygui_api;

extern crate serde;
extern crate serde_json;

use plygui_api::*;

pub use plygui_api::markup;

use std::collections::HashMap;

const ID: &str = "id";
const TYPE: &str = "type";

pub fn parse_markup<F>(json: &str, constructor: F) -> (Box<traits::UiControl>, markup::MarkupIds) where F: Fn(&str, &markup::Markup) -> Box<traits::UiControl> {
	let mut ids: markup::MarkupIds = HashMap::new();
	let markup: markup::Markup = serde_json::from_str(json).unwrap();
	
	let member = parse_node(&markup, &mut ids, constructor);
	(member, ids)
}

fn parse_node<F>(node: &markup::Markup, ids: &mut markup::MarkupIds, constructor: F) -> Box<traits::UiControl> where F: Fn(&str, &markup::Markup) -> Box<traits::UiControl> {
	if let Some(a_type) = node.get(TYPE) {
		let member = match *a_type {
			markup::MarkupNode::Attribute(ref a_type) => constructor(a_type, &node),
			_ => panic!("\"type\" can only be a plain String attribute, {:?} found instead", a_type),
		};
		if let Some(id) = node.get(ID) {
			match *id {
				markup::MarkupNode::Attribute(ref id) => ids.insert(id.clone(), member.id()),
				_ => panic!("\"id\" can only be a plain String attribute, {:?} found instead", id),
			};
		}
		member
	} else {
		panic!("Markup does not have a \"type\": {:?}", node)
	}
}