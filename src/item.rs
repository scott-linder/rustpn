//! Language items and abstract-syntax tree.

/// The equivalent of a routine/function.
pub type Block = Vec<Item>;

/// Any valid value on the stack or in a block.
#[derive(PartialEq, Eq, Clone, Show)]
pub enum Item {
    Call(String),
    Integer(i64),
    String(String),
    Block(Block),
}
