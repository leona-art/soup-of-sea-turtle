use crate::entity::user::{id::UserId, User};
use thiserror::Error;

const MAX_MEMBERS: usize = 5;
#[derive(Error,Debug)]
pub enum MembersError{
    #[error("Members is full. please use {} members or less.",MAX_MEMBERS)]
    Full,
}
#[derive(Debug,Clone,Eq,PartialEq)]
pub struct Members(Vec<User>);

impl Members{
    pub fn new() -> Self{
        Self(Vec::with_capacity(MAX_MEMBERS))
    }
    pub fn add(&mut self,user:User)-> Result<(),MembersError>{
        if self.0.len() >= MAX_MEMBERS{
            return Err(MembersError::Full);
        }
        self.0.push(user);
        Ok(())
    }
    pub fn remove(&mut self,user_id:&UserId){
        self.0.retain(|user| *user.id() != *user_id);
    }
}

impl Iterator for Members{
    type Item = User;
    fn next(&mut self) -> Option<Self::Item>{
        self.0.pop()
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_new(){
        let members = Members::new();
        assert_eq!(members.0.len(),0);
    }
    #[test]
    fn test_add(){
        let mut members = Members::new();
        let user = User::new("test").unwrap();
        members.add(user.clone()).unwrap();
        assert_eq!(members.0.len(),1);
        assert_eq!(members.0[0],user);
    }
    #[test]
    fn test_add_error_full(){
        let mut members = Members::new();
        for _ in 0..MAX_MEMBERS{
            let user = User::new("test").unwrap();
            members.add(user).unwrap();
        }
        let user = User::new("test").unwrap();
        let result = members.add(user);
        assert!(result.is_err());
    }
    #[test]
    fn test_remove(){
        let mut members = Members::new();
        let user = User::new("test").unwrap();
        members.add(user.clone()).unwrap();
        members.remove(user.id());
        assert_eq!(members.0.len(),0);
    }
}