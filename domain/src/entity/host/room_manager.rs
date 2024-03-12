use crate::entity::room::{self, Room};

use super::Host;

pub struct RoomManager{
    pub room:Option<Room>,
    state:RoomState
}

impl  RoomManager{
    pub fn new()->Self{
        RoomManager{
            room:None,
            state:RoomState::Close
        }
    }
    pub fn create_room(&mut self,name:String,host:&Host){
        let mut room = Room::new(name);
        room.add_player(host.into());
        self.room = Some(room);
    }
    pub fn remove_room(&mut self){
        self.room = None;
    }

    pub fn open_room(&mut self){
        self.state = RoomState::Open;
    }

    pub fn close_room(&mut self){
        self.state = RoomState::Close;
    }
}

pub enum RoomState{
    Open,
    Close
}