use std::{cell::Cell, convert::TryInto};

use crate::{Attr, Node, NodeType};

#[derive(Debug)]
pub struct Element<'a> {
	tag_name: &'a str,
}

impl<'a> Element<'a> {
	pub const fn new(tag_name: &'a str) -> Self {
		Self { tag_name }
	}
}
