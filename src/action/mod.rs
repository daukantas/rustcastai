#[derive(Debug, Deserialize)]
pub struct Action {
    slug: String,
    done: bool,
    reply: Option<String>,
}
