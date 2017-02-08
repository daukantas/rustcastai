#[derive(Debug, Deserialize)]
pub struct Action {
    pub slug: String,
    pub done: bool,
    pub reply: Option<String>,
}
