#[derive(Debug, Clone, PartialEq)]
pub enum Types {
    I32,
    I64,
    F32,
    F64,
    Bool,
    String,
    Pointer(Box<Types>),
}
