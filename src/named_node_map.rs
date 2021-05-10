use crate::Attr;
use std::sync::{Arc, Mutex, RwLock};

#[derive(Debug, Clone)]
pub struct NamedNodeMap {
	items: Arc<RwLock<Vec<Arc<Mutex<Attr>>>>>,
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
		let mut items = self.items.write().unwrap();
		items.push(Arc::new(Mutex::new(item)));
	}

	pub fn item(&self, idx: usize) -> Option<Arc<Mutex<Attr>>> {
		let items = self.items.read().unwrap();
		if idx > items.len() {
			None
		} else {
			Some(items[idx].clone())
		}
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
