use std::error::Error;
use super::Game;

pub trait GameRepository {
    fn save(&self, game: Game) -> Result<(), Box<dyn Error>>;
    fn find_by_id(&self, id: &str) -> Result<Option<Game>, Box<dyn Error>>;
}