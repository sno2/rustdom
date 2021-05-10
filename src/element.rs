use crate::NamedNodeMap;
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

	pub fn attributes_lock(&self) -> Arc<RwLock<NamedNodeMap>> {
		self.attrs.clone()
	}
}

#[cfg(test)]
mod tests {
	use super::Element;
	use crate::Attr;

	#[test]
	fn basic_functionality() {
		let el = Element::new("h1");
		let lock = el.attributes_lock();
		{
			let mut attributes = lock.write().unwrap();
			attributes.add(Attr::new("data-name", "carter"));
		}
		assert_eq!(lock.read().unwrap().length(), 1);
	}
}
