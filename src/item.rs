//! Language items and abstract-syntax tree.

/// The equivalent of a routine/function.
pub type Block = Vec<BlockItem>;

/// Language items only valid in a block.
#[derive(PartialEq, Eq, Clone, Show)]
pub enum BlockItem {
    Call(String),
    Literal(StackItem),
}

/// The global stack.
pub type Stack = Vec<StackItem>;

/// Language items only valid on the stack.
#[derive(PartialEq, Eq, Clone, Show)]
pub enum StackItem {
    Integer(i64),
    String(String),
    Boolean(bool),
    Block(Block),
}
