use crate::common::id::Id;

use super::{player::{IsHost, Player}, room::Room};

pub struct Guest {
    pub id:Id,
    pub name:String,
}

impl Guest{
    pub fn new(name:&str)->Self{
        Guest{
            id:Id::new(),
            name:name.to_string(),
        }
    }
}

impl Player for Guest{
    fn id(&self)->Id{
        self.id
    }
    fn name(&self)->String{
        self.name.clone()
    }
}

impl IsHost for Guest{
    fn is_host(&self)->bool{
        false
    }
}