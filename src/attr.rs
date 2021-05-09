use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Debug, Clone)]
pub struct Attr {
	name: Arc<&'static str>,
	value: Arc<Mutex<&'static str>>,
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

impl From<(&'static str, &'static str)> for Attr {
	fn from(src: (&'static str, &'static str)) -> Attr {
		Attr::new(src.0, src.1)
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
			let mut value_guard = attr.value_guard();
			assert_eq!(*value_guard, "text");
			*value_guard = "password";
			assert_eq!(*value_guard, "password");
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
				let guard = attr2.try_lock().unwrap();
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

		let guard = attr.try_lock().unwrap();
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
