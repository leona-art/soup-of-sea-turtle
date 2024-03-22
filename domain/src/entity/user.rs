
pub mod id;
pub mod name;
use std::str::FromStr;

use id::{UserId,UserIdError};
use name::{UserName,UserNameError};
use thiserror::Error;

#[derive(Error,Debug)]
pub enum UserError{
    #[error("User id error: {0}")]
    UserIdError(#[from] UserIdError),
    #[error("User name error: {0}")]
    UserNameError(#[from] UserNameError),
}

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct User{
    id:UserId,
    name:UserName
}

impl User{
    pub fn new(name:&str) -> Result<Self,UserError>{
        let name = UserName::from_str(name)?;
        Ok(Self{
            id:UserId::new(),
            name,
        })
    }
    pub fn id(&self) -> &UserId{
        &self.id
    }
    pub fn name(&self) -> &UserName{
        &self.name
    }
}
