use crate::entity::user::User;


pub struct Players(Vec<User>);
impl Players{
    pub fn new(players:&[User]) -> Self{
        Self(players.to_vec())
    }
}