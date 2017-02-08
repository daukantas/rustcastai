#[derive(Debug, Deserialize)]
pub struct Word {
    pub id: i32,
    pub name: String,
    pub slug: String,
}
