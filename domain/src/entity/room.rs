use crate::common::id::Id;
use super::player::Player;

pub struct Room{
    pub id: Id,
    pub name: String,
    pub players:Vec<Player>
}
impl Room{
    pub fn new(name:String)->Self{
        Room{
            id:Id::new(),
            name,
            players:Vec::new()
        }
    }

    pub fn add_player(&mut self,player:Player){
        self.players.push(player);
    }

    pub fn remove_player(&mut self,player:Player){
        self.players.retain(|p| p.id != player.id);
    }
}