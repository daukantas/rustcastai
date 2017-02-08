#[derive(Debug, Deserialize)]
pub struct Intent {
    pub slug: String,
    pub confidence: f32,
}
