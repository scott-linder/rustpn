//! Language items and abstract-syntax tree.

/// The equivalent of a routine/function.
pub type Block<I> = Vec<BlockItem<I>>;

/// Language items only valid in a block.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum BlockItem<I> {
    Call(String),
    Literal(StackItem<I>),
}

/// The global stack.
pub type Stack<I> = Vec<StackItem<I>>;

/// Language items only valid on the stack.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum StackItem<I> {
    Integer(I),
    String(String),
    Boolean(bool),
    Block(Block<I>),
}
