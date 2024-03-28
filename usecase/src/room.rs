use std::sync::Arc;

use domain::entity::{room::{members::MembersError, repository::RoomRepository, Room, RoomError}, user::{User, UserError}};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use super::user::UserDto;

#[derive(Serialize,Deserialize,Debug)]
pub struct RoomDto{
    id:String,
    name:String,
    members:Vec<UserDto>
}

impl From<Room> for RoomDto{
    fn from(room:Room) -> Self{
        Self{
            id:room.id(),
            name:room.name(),
            members:room.members.into_iter().map(|m|m.into()).collect()
        }
    }
}


#[derive(Error,Debug)]
pub enum CreateRoomError{
    #[error("Room already exists")]
    RoomAlreadyExists,
    #[error("DB error")]
    DBError(#[from] Box<dyn std::error::Error>),
    #[error("Room error: {0}")]
    RoomError(#[from] RoomError),
}

pub struct CreateRoomService<R>
where R:RoomRepository {
    repository: Arc<R>
}

impl <R> CreateRoomService<R>
where R:RoomRepository {
    pub fn new(repository:Arc<R>) -> Self{
        Self{
            repository
        }
    }
    pub fn create_room(&self,name:&str) -> Result<RoomDto,CreateRoomError>{
        let room = Room::new(name)?;
        self.repository.save(room.clone())?;
        Ok(room.into())
    }
}

#[derive(Error,Debug)]
pub enum GetRoomError{
    #[error("Room not found")]
    RoomNotFound,
    #[error("DB error")]
    DBError(#[from] Box<dyn std::error::Error>),
}

pub struct GetRoomService<R>
where R:RoomRepository {
    repository: Arc<R>
}
impl<R> GetRoomService<R>
where R:RoomRepository {
    pub fn new(repository:Arc<R>) -> Self{
        Self{
            repository
        }
    }
    pub fn get_room(&self,id:&str) -> Result<RoomDto,GetRoomError>{
        let room = self.repository.find_by_id(id)?;
        match room{
            Some(room) => Ok(room.into()),
            None => Err(GetRoomError::RoomNotFound)
        }
    }
}

#[derive(Error,Debug)]
pub enum AddMemberError{
    #[error("User not found")]
    UserNotFound,
    #[error("Room not found")]
    RoomNotFound,
    #[error("DB error")]
    DBError(#[from] Box<dyn std::error::Error>),
    #[error("Room error: {0}")]
    RoomError(#[from] RoomError),
    #[error("Members error: {0}")]
    MembersError(#[from] MembersError),
    #[error("Invailed user: {0}")]
    InvailedUser(#[from] UserError),
}

pub struct AddMemberService<R>
where R:RoomRepository {
    repository: Arc<R>
}
impl<R> AddMemberService<R>
where R:RoomRepository {
    pub fn new(repository:Arc<R>) -> Self{
        Self{
            repository
        }
    }
    pub fn add_member(&self,room_id:&str,user:UserDto) -> Result<RoomDto,AddMemberError>{
        let room = self.repository.find_by_id(room_id)?;
        let user = User::from_data(&user.id, &user.name)?;
        match room{
            Some(mut room) => {
                room.members.add(user.clone())?;
                self.repository.save(room.clone())?;
                Ok(room.into())
            },
            None => Err(AddMemberError::RoomNotFound),
        }
    }
}