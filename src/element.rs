use crate::{Attr, NamedNodeMap};
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Element {
	tag_name: Arc<&'static str>,
	attrs: Arc<RwLock<NamedNodeMap>>,
}

impl Element {
	pub fn new(tag_name: &'static str) -> Self {
		Self {
			tag_name: Arc::new(tag_name),
			attrs: Arc::new(RwLock::new(NamedNodeMap::new())),
		}
	}

	/// A lower-level method that obtains the lock to read or write
	/// to the [`NamedNodeMap`] that holds the attributes for this
	/// element.
	pub fn attributes_lock(&self) -> Arc<RwLock<NamedNodeMap>> {
		self.attrs.clone()
	}

	pub fn get_attribute(&self, name: &'static str) -> Option<&'static str> {
		let lock = self.attributes_lock();
		let map = lock.read().unwrap();
		let item = map.get_named_item(name);
		return match item {
			Some(attr) => Some(attr.read().unwrap().value()),
			_ => None,
		};
	}

	pub fn set_attribute(&mut self, name: &'static str, value: &'static str) {
		let lock = self.attributes_lock();
		let mut map = lock.write().unwrap();
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
			let mut attributes = lock.write().unwrap();
			attributes.add(Attr::new("data-name", "carter"));
		}
		assert_eq!(lock.read().unwrap().length(), 1);
	}

	#[test]
	fn attributes_helpers() {
		let mut el = Element::new("h1");
		// HANGING HERE!
		el.set_attribute("type", "text");

		assert_eq!(el.get_attribute("type").unwrap(), "text");
	}
}
