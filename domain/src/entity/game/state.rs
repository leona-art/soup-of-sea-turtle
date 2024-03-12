pub enum GameState {
    Setup,
    Play,
    End,
}

impl GameState{
    pub fn new() -> Self {
        GameState::Setup
    }
    pub fn next(&self) -> Self {
        match self {
            GameState::Setup => GameState::Play,
            GameState::Play => GameState::End,
            GameState::End => GameState::Setup,
        }
    }
}