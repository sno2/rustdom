#[derive(Debug)]
pub enum NodeType {
	ElementNode = 1,
	AttributeNode,
	TextNode,
	CDataSectionNode,
	ProcessingInstructionNode = 7,
	CommentNode,
	DocumentNode,
	DocumentTypeNode,
	DocumentFragmentNode,
}
