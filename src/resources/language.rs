#[derive(Debug, Deserialize)]
pub struct Language {
    pub id: i32,
    pub isocode: String,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub is_activated: Option<bool>,
}
