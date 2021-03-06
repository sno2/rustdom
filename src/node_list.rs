use crate::Node;
use std::sync::{Arc, RwLock};

/// [`NodeList`] is inherently thread-safe. Therefore you do not actually need
/// to wrap it in thread-safe mutation containers (like [`Mutex`] or [`RwLock`])
/// to keep it thread-safe.
#[derive(Debug)]
pub struct NodeList<T: Node> {
	items: Arc<RwLock<Vec<Arc<RwLock<T>>>>>,
}

impl<T: Node> NodeList<T> {
	pub fn new() -> Self {
		Self {
			items: Arc::new(RwLock::new(Vec::new())),
		}
	}

	pub fn length(&self) -> usize {
		let guard = self.items.read().unwrap();
		guard.len()
	}

	pub fn item(&self, i: usize) -> Option<Arc<RwLock<T>>> {
		let guard = self.items.read().unwrap();
		match guard.get(i) {
			Some(val) => Some(val.clone()),
			None => None,
		}
	}

	#[allow(dead_code)]
	pub(crate) fn add_raw(&self, item: Arc<RwLock<T>>) {
		let mut guard = self.items.write().unwrap();
		guard.push(item);
	}

	#[allow(dead_code)]
	pub(crate) fn add(&self, item: T) {
		self.add_raw(Arc::new(RwLock::new(item)));
	}
}

#[cfg(test)]
mod tests {
	use std::{
		sync::{Arc, Mutex},
		thread,
	};

	use crate::Attr;

	use super::NodeList;

	#[test]
	fn lengths() {
		let list = NodeList::new();
		assert_eq!(list.length(), 0);
		let attr = Attr::new("data-age", "23");
		list.add(attr);
		assert_eq!(list.length(), 1);
	}

	#[test]
	fn invalid_indices() {
		let list: NodeList<Attr> = NodeList::new();
		if list.item(0).is_some() {
			panic!("The list should not have any items.");
		}
		let attr = Attr::new("type", "text");
		list.add(attr);
		if list.item(0).is_none() {
			panic!("The node list should have a single node.");
		}
	}

	#[test]
	fn elements_maintain_state() {
		let list: NodeList<Attr> = NodeList::new();
		let attr = Attr::new("type", "text");
		list.add(attr.clone());
		assert_eq!(attr.value(), "text");
		list.item(0).unwrap().write().unwrap().set_value("password");
		assert_eq!(attr.value(), "password");
	}

	#[test]
	fn multithreaded_mutations() {
		const ITERS: usize = 10;
		let list = Arc::new(Mutex::new(NodeList::new()));
		let mut handlers = vec![];

		for i in 0..ITERS {
			let list = list.clone();
			let handler = thread::spawn(move || {
				let list = list.lock().unwrap();
				list.add(Attr::new(format!("data-{}", i), ""));
			});
			handlers.push(handler);
		}

		for handler in handlers {
			handler.join().unwrap();
		}

		for i in 0..ITERS {
			let list = list.lock().unwrap();
			list.item(i)
				.expect("Attribute not added with multithreading.");
		}

		assert_eq!(list.lock().unwrap().length(), ITERS);
	}
}
