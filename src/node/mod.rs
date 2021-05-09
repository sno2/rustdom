mod node_type;

pub use node_type::NodeType;

pub trait Node {
	type Value;
	type Child: Node;

	fn namespace_uri(&self) -> Option<&'static str>;
	fn child_nodes<T: Node>(&self) -> Option<&'static [T]>;
	fn first_child<T: Node>(&self) -> Option<T>;
	fn is_connected(&self) -> bool;
	fn last_child<T: Node>(&self) -> Option<T>;
	fn next_sibling<T: Node>(&self) -> Option<T>;
	fn node_name(&self) -> &'static str;
	fn node_type(&self) -> NodeType;
	fn node_value(&self) -> Self::Value;
	fn set_node_value(&mut self, new_value: Self::Value);
	// Use Document:
	fn owner_document<T: Node>(&self) -> T;
	fn parent_node<T: Node>(&self) -> Option<T>;
	fn parent_element<T: Node>(&self) -> Option<T>;
	fn previous_sibling<T: Node>(&self) -> Option<T>;
	fn text_content(&self) -> &'static str;
	fn set_text_content(&self, content: &'static str);
	fn append_child<T: Node>(&mut self, child: T);
	fn clone_node(&self) -> Self;
	// compare_document_position
	fn contains(&self, child: Self::Child) -> bool;
	fn get_root_node<T: Node>(&self) -> T;
	fn has_child_nodes(&self) -> bool;
	fn insert_before<T: Node>(&mut self, before_node: T);
	fn is_default_namespace(&self) -> bool;
	fn is_equal_node<T: Node>(&self, other: T) -> bool;
	fn is_same_node(&self, other: Self) -> bool;
	fn lookup_prefix(&self) -> Option<&'static str>;
	fn lookup_namespace_uri(&self, prefix: &'static str) -> Option<&'static str>;
	fn normalize(&mut self);
	fn remove_child(&mut self, child: Self::Child);
	fn replace_child(&mut self, new_child: Self::Child, old_child: Self::Child);
}
