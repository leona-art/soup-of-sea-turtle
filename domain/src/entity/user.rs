
pub mod id;
pub mod name;
pub mod repository;
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
    pub(super) id:UserId,
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
    pub fn id(&self) -> String{
        self.id.to_string()
    }
    pub fn name(&self) -> String{
        self.name.to_string()
    }
    pub fn from_data(id:&str,name:&str) -> Result<Self,UserError>{
        (id,name).try_into()
    }
}

impl TryFrom<(&str,&str)> for User{
    type Error = UserError;
    fn try_from(data:(&str,&str)) -> Result<Self,Self::Error>{
        let name = UserName::from_str(data.1)?;
        Ok(Self{
            id:UserId::try_from(data.0)?,
            name
        })
    }
}