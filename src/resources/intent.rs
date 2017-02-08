use super::Expression;

#[derive(Debug, Deserialize)]
pub struct Intent {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub expressions: Option<Vec<Expression>>,
}
