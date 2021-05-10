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
