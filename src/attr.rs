use crate::{Node, NodeType};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct Attr {
	name: Arc<&'static str>,
	value: Arc<RwLock<&'static str>>,
}

impl Attr {
	/// Creates a new [`Attr`] with the given name and value.
	pub fn new(name: &'static str, value: &'static str) -> Self {
		Self {
			name: Arc::new(name),
			value: Arc::new(RwLock::new(value)),
		}
	}

	/// Gets the value of the attribute.
	pub fn name(&self) -> &'static str {
		*self.name
	}

	/// Gets the value of the attribute.
	pub fn value(&self) -> &'static str {
		*self.value.read().unwrap()
	}

	/// Sets the value of the attribute.
	pub fn set_value(&self, new_value: &'static str) {
		let mut value = self.value.write().unwrap();
		*value = new_value;
	}
}

impl From<(&'static str, &'static str)> for Attr {
	fn from(src: (&'static str, &'static str)) -> Attr {
		Attr::new(src.0, src.1)
	}
}

impl Node for Attr {
	type Value = &'static str;
	type Child = Attr;

	fn namespace_uri(&self) -> Option<&'static str> {
		None
	}

	fn child_nodes<T: Node>(&self) -> Option<&'static [T]> {
		None
	}

	fn first_child<T: Node>(&self) -> Option<T> {
		None
	}

	fn is_connected(&self) -> bool {
		todo!() // not on mdn
	}

	fn last_child<T: Node>(&self) -> Option<T> {
		None
	}

	fn next_sibling<T: Node>(&self) -> Option<T> {
		None
	}

	fn node_name(&self) -> &'static str {
		self.name()
	}

	fn node_type(&self) -> NodeType {
		NodeType::AttributeNode
	}

	fn node_value(&self) -> &'static str {
		self.value()
	}

	fn set_node_value(&mut self, new_value: &'static str) {
		self.set_value(new_value);
	}

	fn owner_document<T: Node>(&self) -> T {
		unimplemented!()
	}

	fn parent_node<T: Node>(&self) -> Option<T> {
		None
	}

	fn parent_element<T: Node>(&self) -> Option<T> {
		todo!()
	}

	fn previous_sibling<T: Node>(&self) -> Option<T> {
		None
	}

	fn text_content(&self) -> &'static str {
		self.value()
	}

	fn set_text_content(&self, new_content: &'static str) {
		self.set_value(new_content);
	}

	fn append_child<T: Node>(&mut self, child: T) {
		unimplemented!()
	}

	fn clone_node(&self) -> Self::Child {
		unimplemented!()
	}

	fn contains(&self, child: Self::Child) -> bool {
		unimplemented!()
	}

	fn get_root_node<T: Node>(&self) -> T {
		unimplemented!()
	}

	fn has_child_nodes(&self) -> bool {
		unimplemented!()
	}

	fn insert_before<T: Node>(&mut self, before_node: T) {
		unimplemented!()
	}

	fn is_default_namespace(&self) -> bool {
		unimplemented!()
	}

	fn is_equal_node<T: Node>(&self, other: T) -> bool {
		unimplemented!()
	}

	fn is_same_node(&self, other: Self) -> bool {
		unimplemented!()
	}

	fn lookup_prefix(&self) -> Option<&'static str> {
		todo!()
	}

	fn lookup_namespace_uri(&self, prefix: &'static str) -> Option<&'static str> {
		todo!()
	}

	fn normalize(&mut self) {
		todo!()
	}

	fn remove_child(&mut self, child: Self::Child) {
		todo!()
	}

	fn replace_child(&mut self, new_child: Self::Child, old_child: Self::Child) {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::Attr;
	use std::{
		sync::{Arc, Mutex},
		thread,
	};

	#[test]
	fn basic_binding() {
		let attr = Attr::new("type", "text");
		assert_eq!("type", attr.name());
		assert_eq!("text", attr.value());
	}

	#[test]
	fn single_threaded_mutations() {
		let attr = Attr::new("type", "text");
		assert_eq!("text", attr.value());
		attr.set_value("password");
		assert_eq!("password", attr.value());
	}

	#[test]
	fn single_threaded_guards() {
		let attr = Attr::new("type", "text");
		{
			assert_eq!(attr.value(), "text");
			attr.set_value("password");
			assert_eq!(attr.value(), "password");
		}
		assert_eq!(attr.value(), "password");
	}

	#[test]
	fn multithreaded_mutations() {
		let attr = Arc::new(Mutex::new(Attr::new("type", "text")));

		let mut handlers = vec![];

		let mut is_last = false;
		for i in 0..2 {
			let attr2 = attr.clone();
			handlers.push(thread::spawn(move || {
				let guard = attr2.lock().unwrap();
				if is_last {
					assert_eq!(guard.value(), "foo");
				} else {
					guard.set_value("foo");
					is_last = true;
				}
			}));
		}

		for handler in handlers {
			handler.join().unwrap();
		}

		let guard = attr.lock().unwrap();
		assert_eq!(guard.name(), "type");
		assert_eq!(guard.value(), "foo");
	}

	#[test]
	fn from_trait_implementation() {
		let attr = Attr::from(("type", "text"));
		assert_eq!(attr.name(), "type");
		assert_eq!(attr.value(), "text");
	}
}
