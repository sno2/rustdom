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
		let attr2 = attr.clone();

		let handler = thread::spawn(move || {
			let guard = attr2.try_lock().unwrap();
			assert_eq!("text", guard.value());
			guard.set_value("foo");
		});

		handler.join().unwrap();

		let guard = attr.try_lock().unwrap();
		assert_eq!(guard.name(), "type");
		assert_eq!(guard.value(), "foo");
	}
}
