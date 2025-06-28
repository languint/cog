#[derive(Debug, Clone, PartialEq)]
pub enum Nodes {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
}
