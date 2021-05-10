use crate::Attr;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct NamedNodeMap {
	pub(crate) items: Arc<RwLock<Vec<Arc<RwLock<Attr>>>>>,
}

impl NamedNodeMap {
	pub fn new() -> Self {
		Self {
			items: Arc::new(RwLock::new(Vec::new())),
		}
	}

	pub fn length(&self) -> usize {
		self.items.read().unwrap().len()
	}

	pub(crate) fn add(&mut self, item: Attr) {
		let lock = self.items_lock();
		let mut items = lock.write().unwrap();
		items.push(Arc::new(RwLock::new(item)));
	}

	fn items_lock(&self) -> Arc<RwLock<Vec<Arc<RwLock<Attr>>>>> {
		self.items.clone()
	}

	pub fn item(&self, idx: usize) -> Option<Arc<RwLock<Attr>>> {
		let items = self.items.read().unwrap();
		if idx > items.len() {
			None
		} else {
			Some(items[idx].clone())
		}
	}

	pub fn get_named_item(&self, name: &'static str) -> Option<Arc<RwLock<Attr>>> {
		let lock = self.items_lock();
		let items = lock.read().unwrap();

		for item in items.iter() {
			if item.read().unwrap().name() == name {
				return Some(item.clone());
			}
		}
		None
	}

	pub fn set_named_item(&mut self, attr: Attr) {
		let lock = self.items_lock();
		let items = lock.write().unwrap();
		let name = attr.name();
		for item_lock in items.iter() {
			let mut item_guard = item_lock.write().unwrap();
			if item_guard.name() == name {
				*item_guard = attr;
				return;
			}
		}
		self.add(attr);
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

	const ITER_COUNT: usize = 25;

	#[test]
	fn singlethreaded_mutations() {
		let store = Arc::new(Mutex::new(NamedNodeMap::new()));

		for _ in 0..ITER_COUNT {
			let mut map = store.try_lock().unwrap();
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
				let mut map = store.lock().unwrap();
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
