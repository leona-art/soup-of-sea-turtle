#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PlayerName(String);

impl PlayerName {
    pub fn new(name: &str) -> PlayerName {
        PlayerName(name.to_string())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}