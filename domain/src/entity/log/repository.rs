use std::error::Error;

use super::GameLog;

pub trait LogRepository {
    fn save(&self, log: GameLog) -> Result<(), Box<dyn Error>>;
    fn find_by_id(&self, id: &str) -> Result<Option<GameLog>, Box<dyn Error>>;
}