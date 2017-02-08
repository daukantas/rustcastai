use super::{Language, Token};

#[derive(Debug, Deserialize)]
pub struct Expression {
    pub id: i32,
    pub source: String,
    pub language: Language,
    pub tokens: Option<Vec<Token>>,
}
