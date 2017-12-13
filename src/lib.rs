extern crate plygui_api;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use plygui_api::*;

use std::collections::HashMap;

const ID: &str = "id";
const TYPE: &str = "type";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MarkupNode {
	Attribute(String),
	Child(Markup),
	Children(Vec<Markup>),
}

pub type Markup = HashMap<String, MarkupNode>;
pub type MarkupIds = HashMap<String, ids::Id>;

pub fn parse_markup<F>(json: &str, constructor: F) -> (Box<traits::UiMember>, MarkupIds) where F: Fn(&str, &Markup) -> Box<traits::UiMember> {
	let mut ids: MarkupIds = HashMap::new();
	let markup: Markup = serde_json::from_str(json).unwrap();
	
	let member = parse_node(&markup, &mut ids, constructor);
	(member, ids)
}

fn parse_node<F>(node: &Markup, ids: &mut MarkupIds, constructor: F) -> Box<traits::UiMember> where F: Fn(&str, &Markup) -> Box<traits::UiMember> {
	if let Some(a_type) = node.get(TYPE) {
		let member = match *a_type {
			MarkupNode::Attribute(ref a_type) => constructor(a_type, &node),
			_ => panic!("\"type\" can only be a plain String attribute, {:?} found instead", a_type),
		};
		if let Some(id) = node.get(ID) {
			match *id {
				MarkupNode::Attribute(ref id) => ids.insert(id.clone(), member.id()),
				_ => panic!("\"id\" can only be a plain String attribute, {:?} found instead", id),
			};
		}
		member
	} else {
		panic!("Markup does not have a \"type\": {:?}", node)
	}
}