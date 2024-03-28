use crate::entity::user::User;
use thiserror::Error;



#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Player {
    user: User,
    role: Role,
}

impl Player {
    pub fn new(user: User,role:Role) -> Self {
        Self {
            user,
            role,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Role {
    Questioner,
    Guesser,
}

#[derive(Error, Debug)]
pub enum PlayersError {
    #[error("Questioner is already exist")]
    QuestionerIsAlreadyExist,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Players(Vec<Player>);
impl Players {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn add(&mut self, player: Player)->Result<(),PlayersError> {
        let questioner_count=self.0.iter().filter(|p|p.role==Role::Questioner).count();
        if player.role==Role::Questioner && questioner_count>0{
            return Err(PlayersError::QuestionerIsAlreadyExist);
        }
        self.0.push(player);
        Ok(())
    }
    pub fn from_data(data: &[(User,Role)]) -> Result<Self,PlayersError> {
        data.try_into()
    }
}
impl TryFrom<&[(User,Role)]> for Players {
    type Error = PlayersError;
    fn try_from(data: &[(User,Role)]) -> Result<Self,Self::Error> {
        let mut players=Players::new();
        for (user,role) in data {
            let player=Player::new(user.clone(),role.clone());
            players.add(player)?;
        }
        Ok(players)
    }
}



impl IntoIterator for Players {
    type Item = Player;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_from(){
        let user_roles=vec![(User::new("user1").unwrap(),Role::Questioner),(User::new("user2").unwrap(),Role::Guesser)];
        let players=Players::try_from(&user_roles[..]);
    }
}