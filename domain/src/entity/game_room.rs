use crate::common::id::Id;

use super::{game::Game, player::Player};

/// ゲームルームエンティティ  
/// ゲームルームはプレイヤーを管理する
pub struct GameRoom {
    pub id: Id,
    pub name: String,
    pub players: Vec<Player>,
    pub game:Option<Game>,
}

impl GameRoom {
    pub fn new(name: String) -> GameRoom {
        GameRoom {
            id:Id::new(),
            name,
            players: Vec::new(),
            game:None,
        }
    }
    
    /// プレイヤーの追加メソッド
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
    pub fn remove_player(&mut self, player_id: &Id) {
        self.players.retain(|p| p.id.value() != player_id.value());
    }
}
