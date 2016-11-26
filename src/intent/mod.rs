#[derive(Debug, Deserialize)]
pub struct Intent {
    slug: String,
    confidence: f32,
}
