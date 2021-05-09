#[derive(Debug)]
pub enum NodeType {
	ElementNode = 1,
	TextNode = 3,
	CDataSectionNode,
	ProcessingInstructionNode = 7,
	CommentNode,
	DocumentNode,
	DocumentTypeNode,
	DocumentFragmentNode,
}
