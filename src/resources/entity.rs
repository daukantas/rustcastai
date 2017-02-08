#[derive(Debug, Deserialize)]
pub struct Entity {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub color: String,
    pub custom: bool,
}
