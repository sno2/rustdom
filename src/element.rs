#[derive(Debug)]
pub struct Element<'a> {
	tag_name: &'a str,
}

impl<'a> Element<'a> {
	pub const fn new(tag_name: &'a str) -> Self {
		Self { tag_name }
	}
}
