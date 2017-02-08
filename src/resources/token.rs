use super::{Word, Entity};

#[derive(Debug, Deserialize)]
pub struct Token {
    pub id: i32,
    pub space: bool,
    pub word: Word,
    pub entity: Entity,
}
