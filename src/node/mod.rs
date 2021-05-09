mod node_type;

pub use node_type::NodeType;

pub trait Node<'a> {
	fn base_uri(&self) -> &'static str;
	fn child_nodes<T: Node<'static>>(&self) -> &'static [T];
	fn first_child<T: Node<'static>>(&self) -> T;
	fn is_connected(&self) -> bool;
	fn last_child<T: Node<'static>>(&self) -> T;
	fn next_sibling<T: Node<'static>>(&self) -> T;
	fn node_name(&self) -> &'a str;
	fn node_type(&self) -> NodeType;
	fn node_value(&self) -> Option<&'static str>;
	fn set_node_value(&mut self);
	// Use Document:
	fn owner_document<T: Node<'static>>(&self) -> T;
	fn parent_node<T: Node<'static>>(&self) -> Option<T>;
	fn parent_element<T: Node<'static>>(&self) -> Option<T>;
	fn previous_sibling<T: Node<'static>>(&self) -> Option<T>;
	fn text_content(&self) -> &'static str;
	fn set_text_content(&self, content: &'static str);
	fn append_child<T: Node<'static>>(&mut self, child: T);
	fn clone_node<T: Node<'static>>(&self) -> T;
	// compare_document_position
	fn contains<T: Node<'static>>(&self, child: T) -> bool;
	fn get_root_node<T: Node<'static>>(&self) -> T;
	fn has_child_nodes(&self) -> bool;
	fn insert_before<T: Node<'static>>(&mut self, before_node: T);
	fn is_default_namespace(&self) -> bool;
	fn is_equal_node<T: Node<'static>>(&self, other: T) -> bool;
	fn is_same_node(&self, other: Self) -> bool;
	fn lookup_prefix(&self) -> Option<&'static str>;
	fn lookup_namespace_uri(&self, prefix: &'static str) -> Option<&'static str>;
	fn normalize(&mut self);
	fn remove_child<T: Node<'static>>(&mut self, child: T);
	fn replace_child<T: Node<'static>>(&mut self, new_child: T, old_child: T);
}
