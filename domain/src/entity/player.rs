use std::fmt::Debug;

use crate::common::{id::Id, player_name::PlayerName};

use super::{guest::Guest, host::Host};


pub trait Player
where Self:Debug+Clone+PartialEq+Eq+Sized
{
    fn id(&self) -> Id;
    fn name(&self) -> PlayerName;
}

pub trait IsHost {
    fn is_host(&self) -> bool;
}