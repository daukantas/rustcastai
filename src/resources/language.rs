#[derive(Debug, Deserialize)]
pub struct Language {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub isocode: String,
    pub is_activated: bool,
}
