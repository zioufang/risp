#[derive(Debug, PartialEq)]
pub enum LispObj {
    Void,
    Integer(i64),
    Bool(bool),
    Symbol(String),
    Lambda(Vec<String>, Vec<LispObj>),
    List(Vec<LispObj>),
}
