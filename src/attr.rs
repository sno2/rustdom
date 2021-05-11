use crate::{Node, NodeType};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct Attr {
	name: Arc<String>,
	value: Arc<RwLock<String>>,
}

impl Attr {
	/// Creates a new [`Attr`] with the given name and value.
	pub fn new<T, F>(name: T, value: F) -> Self
	where
		T: Into<String>,
		F: Into<String>,
	{
		Self {
			name: Arc::new(name.into()),
			value: Arc::new(RwLock::new(value.into())),
		}
	}

	/// Gets the value of the attribute.
	pub fn name(&self) -> String {
		(*self.name).clone()
	}

	/// Gets the value of the attribute.
	pub fn value(&self) -> String {
		(*self.value.read().unwrap()).clone()
	}

	/// Sets the value of the attribute.
	pub fn set_value<T: Into<String>>(&self, new_value: T) {
		let mut value = self.value.write().unwrap();
		*value = new_value.into();
	}
}

impl Node for Attr {
	type Value = String;
	type Child = Attr;

	fn namespace_uri(&self) -> Option<String> {
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

	fn node_name(&self) -> String {
		self.name()
	}

	fn node_type(&self) -> NodeType {
		NodeType::AttributeNode
	}

	fn node_value(&self) -> String {
		self.value()
	}

	fn set_node_value(&mut self, new_value: String) {
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

	fn text_content(&self) -> String {
		self.value()
	}

	fn set_text_content(&self, new_content: String) {
		self.set_value(new_content);
	}

	fn append_child<T: Node>(&mut self, _child: T) {
		unimplemented!()
	}

	fn clone_node(&self) -> Self::Child {
		unimplemented!()
	}

	fn contains(&self, _child: Self::Child) -> bool {
		unimplemented!()
	}

	fn get_root_node<T: Node>(&self) -> T {
		unimplemented!()
	}

	fn has_child_nodes(&self) -> bool {
		unimplemented!()
	}

	fn insert_before<T: Node>(&mut self, _before_node: T) {
		unimplemented!()
	}

	fn is_default_namespace(&self) -> bool {
		unimplemented!()
	}

	fn is_equal_node<T: Node>(&self, _other: T) -> bool {
		unimplemented!()
	}

	fn is_same_node(&self, _other: Self) -> bool {
		unimplemented!()
	}

	fn lookup_prefix(&self) -> Option<String> {
		todo!()
	}

	fn lookup_namespace_uri(&self, _prefix: String) -> Option<String> {
		todo!()
	}

	fn normalize(&mut self) {
		todo!()
	}

	fn remove_child(&mut self, _child: Self::Child) {
		todo!()
	}

	fn replace_child(&mut self, _new_child: Self::Child, _old_child: Self::Child) {
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
		assert_eq!(attr.value(), "text");
		attr.set_value("password");
		assert_eq!(attr.value(), "password");
		assert_eq!(attr.value(), "password");
	}

	#[test]
	fn multithreaded_mutations() {
		let attr = Arc::new(Mutex::new(Attr::new("type", "text")));

		let mut handlers = vec![];

		for _ in 0..2 {
			let attr2 = attr.clone();
			handlers.push(thread::spawn(move || {
				let guard = attr2.lock().unwrap();
				if guard.value() != "foo" {
					guard.set_value("foo");
				}
			}));
		}

		for handler in handlers {
			handler.join().unwrap();
		}

		assert_eq!(attr.lock().unwrap().value(), "foo");

		let guard = attr.lock().unwrap();
		assert_eq!(guard.name(), "type");
		assert_eq!(guard.value(), "foo");
	}
}
