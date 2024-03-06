pub enum GameState {
    Setup,
    Play,
    End,
}

impl GameState{
    pub fn new() -> GameState {
        GameState::Setup
    }
    pub fn next(&self) -> GameState {
        match self {
            GameState::Setup => GameState::Play,
            GameState::Play => GameState::End,
            GameState::End => GameState::Setup,
        }
    }
}