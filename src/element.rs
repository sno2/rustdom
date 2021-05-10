use crate::{Attr, NamedNodeMap};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct Element {
	tag_name: Arc<&'static str>,
	attrs: Arc<RwLock<NamedNodeMap>>,
}

impl Element {
	/// Creates a new [`Element`] given the tag name.
	pub fn new(tag_name: &'static str) -> Self {
		Self {
			tag_name: Arc::new(tag_name),
			attrs: Arc::new(RwLock::new(NamedNodeMap::new())),
		}
	}

	/// A lower-level method that obtains the lock to read or write to the
	/// [`NamedNodeMap`] that holds the attributes for this element.
	pub fn attributes_lock(&self) -> Arc<RwLock<NamedNodeMap>> {
		self.attrs.clone()
	}

	/// Gets the value of an attribute given its name. Due to attributes not
	/// always having a value, it returns an [`Option`] that might include the
	/// value.
	pub fn get_attribute(&self, name: &'static str) -> Option<&'static str> {
		let lock = self.attributes_lock();
		let map = lock.read().unwrap();
		let item = map.get_named_item(name);
		match item {
			Some(attr) => Some(attr.read().unwrap().value()),
			_ => None,
		}
	}

	/// Sets the value of an attribute given the name and value.
	pub fn set_attribute(&self, name: &'static str, value: &'static str) {
		let lock = self.attributes_lock();
		let map = lock.write().unwrap();
		map.set_named_item(Attr::new(name, value));
	}
}

#[cfg(test)]
mod tests {
	use super::Element;
	use crate::Attr;

	#[test]
	fn mutating_attributes_lock() {
		let el = Element::new("h1");
		let lock = el.attributes_lock();
		{
			let attributes = lock.write().unwrap();
			attributes.add(Attr::new("data-name", "carter"));
		}
		assert_eq!(lock.read().unwrap().length(), 1);
	}

	#[test]
	fn attributes_helpers() {
		let el = Element::new("h1");
		match el.get_attribute("type") {
			None => (),
			_ => panic!("Element should not have an attribute"),
		}
		el.set_attribute("type", "text");
		assert_eq!(el.get_attribute("type").unwrap(), "text");
	}
}
