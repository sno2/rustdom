use crate::{Node, NodeType};
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Debug, Clone)]
pub struct Attr {
	pub(crate) name: Arc<&'static str>,
	pub(crate) value: Arc<Mutex<&'static str>>,
}

impl Attr {
	pub fn new(name: &'static str, value: &'static str) -> Self {
		Self {
			name: Arc::new(name),
			value: Arc::new(Mutex::new(value)),
		}
	}

	pub fn name(&self) -> &'static str {
		*self.name
	}

	/// Gets the value of attribute at the time of function call.
	///
	/// Note: Although this does not stop you from mutating dereferenced value from
	/// the mutex guard, you should abstain from doing so. Instead, use [`Attr.set_value`].
	pub fn value(&self) -> &'static str {
		*self.value_guard()
	}

	pub fn value_guard(&self) -> MutexGuard<&'static str> {
		self.value
			.try_lock()
			.expect("Unable to get mutex guard from attribute.")
	}

	pub fn set_value(&self, new_value: &'static str) {
		let mut guard = self.value_guard();
		*guard = new_value;
	}
}
