use crate::Attr;
use std::{
	borrow::BorrowMut,
	ops::{Deref, DerefMut},
	sync::{Arc, Mutex, MutexGuard},
};

#[derive(Debug, Clone)]
pub struct NamedNodeMap {
	items: Arc<Mutex<Vec<Arc<Mutex<Attr>>>>>,
}

impl NamedNodeMap {
	pub fn new() -> Self {
		Self {
			items: Arc::new(Mutex::new(Vec::new())),
		}
	}

	pub fn length(&self) -> usize {
		self.items.try_lock().unwrap().len()
	}

	pub(crate) fn add(&mut self, item: Attr) {
		let mut items = self.items.try_lock().unwrap();
		items.push(Arc::new(Mutex::new(item)));
	}

	pub fn item(&self, idx: usize) -> Option<Arc<Mutex<Attr>>> {
		let items = self.items.try_lock().unwrap();
		if idx > items.len() {
			None
		} else {
			Some(items[idx].clone())
		}
	}
}
