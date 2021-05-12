use crate::Attr;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct NamedNodeMap {
	pub(crate) items: Arc<RwLock<Vec<Arc<RwLock<Attr>>>>>,
}

impl NamedNodeMap {
	/// Creates a new, empty node map.
	pub fn new() -> Self {
		Self {
			items: Arc::new(RwLock::new(Vec::new())),
		}
	}

	/// Gets the number of items within the current node map.
	pub fn length(&self) -> usize {
		self.items.read().unwrap().len()
	}

	/// Adds a new [`Attr`] into the node map.
	#[allow(dead_code)]
	pub(crate) fn add(&self, item: Attr) {
		self.add_raw(Arc::new(RwLock::new(item)));
	}

	#[allow(dead_code)]
	pub(crate) fn add_raw(&self, item: Arc<RwLock<Attr>>) {
		let lock = self.items_lock();
		let mut items = lock.write().unwrap();
		items.push(item);
	}

	/// Gets the lock for accessing a vector that includes all of the items.
	fn items_lock(&self) -> Arc<RwLock<Vec<Arc<RwLock<Attr>>>>> {
		self.items.clone()
	}

	/// Gets the node at the given index within the node map.
	pub fn item(&self, idx: usize) -> Option<Arc<RwLock<Attr>>> {
		let items = self.items.read().unwrap();
		if idx > items.len() {
			None
		} else {
			Some(items[idx].clone())
		}
	}

	/// Gets the node by its corresponding name within the node map.
	pub fn get_named_item<T: Into<String>>(&self, name: T) -> Option<Arc<RwLock<Attr>>> {
		let lock = self.items_lock();
		let items = lock.read().unwrap();
		let normalized_name: String = name.into();
		for item in items.iter() {
			if item.read().unwrap().name() == normalized_name {
				return Some(item.clone());
			}
		}
		None
	}

	/// Either adds or replaces the existing [`Attr`] depending on whether there
	/// is another node within the map identified by the same name.
	pub fn set_named_item(&self, attr: Attr) {
		let lock = self.items_lock();
		let mut items = lock.write().unwrap();
		let name = attr.name();
		for item_lock in items.iter() {
			let mut item_guard = item_lock.write().unwrap();
			if item_guard.name() == name {
				*item_guard = attr;
				return;
			}
		}
		items.push(Arc::new(RwLock::new(attr)));
	}

	/// Removes a node from the node map by its identifiable [`Attr`]
	/// implementation.
	pub fn remove_named_item(&self, _attr: Attr) {
		todo!();
	}
}

#[cfg(test)]
mod tests {
	use super::NamedNodeMap;
	use crate::Attr;
	use std::{
		sync::{Arc, Mutex},
		thread,
	};

	#[test]
	fn setting_attributes() {
		let store = NamedNodeMap::new();
		store.set_named_item(Attr::new("data-age", "forever"));
		assert_eq!(store.length(), 1);
	}

	#[test]
	fn flushing_duplicates() {
		let store = NamedNodeMap::new();
		store.set_named_item(Attr::new("data-age", "forever"));
		store.set_named_item(Attr::new("data-age", "never"));
		assert_eq!(store.length(), 1);
		assert_eq!(store.item(0).unwrap().read().unwrap().value(), "never");
	}

	const ITER_COUNT: usize = 25;

	#[test]
	fn singlethreaded_mutations() {
		let store = Arc::new(Mutex::new(NamedNodeMap::new()));

		for _ in 0..ITER_COUNT {
			let map = store.try_lock().unwrap();
			map.add(Attr::new("type", "text"));
		}

		assert_eq!(store.try_lock().unwrap().length(), ITER_COUNT);
	}

	#[test]
	fn multithreaded_mutations() {
		let store = Arc::new(Mutex::new(NamedNodeMap::new()));
		let mut handlers = vec![];

		for _ in 0..ITER_COUNT {
			// We need to clone to stop the thread closure from moving the original store.
			let store = store.clone();
			let handler = thread::spawn(move || {
				let map = store.lock().unwrap();
				map.add(Attr::new("type", "text"));
			});
			handlers.push(handler);
		}

		for handle in handlers {
			handle.join().unwrap();
		}

		assert_eq!(store.lock().unwrap().length(), ITER_COUNT);
	}
}
