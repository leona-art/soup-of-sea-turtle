mod error;
use self::error::PartyError;
use super::member::Member;
use crate::common::id::Id;

const MIN_PARTY_MEMBERS: usize = 2;
const MAX_PARTY_MEMBERS: usize = 10;

const QUESTIONER_COUNT: usize = 1;
const MIN_GUESSER_COUNT: usize = 1;

pub struct Party {
    pub id: Id,
    pub members: Vec<Member>,
}

impl Party {
    pub fn new(members: &[Member]) -> Result<Party, PartyError> {
        // パーティーメンバーの数チェック
        if members.len() < MIN_PARTY_MEMBERS || members.len() > MAX_PARTY_MEMBERS {
            return Err(PartyError::InvalidMembersCount);
        }

        // パーティメンバーの役割チェック
        // Questionerが1人、Guesserが1人以上いるか
        let questioner_count = members
            .iter()
            .filter(|r| matches!(r, Member::Questioner(_)))
            .count();
        let guesser_count = members
            .iter()
            .filter(|r| matches!(r, Member::Guesser(_)))
            .count();
        if questioner_count != QUESTIONER_COUNT || guesser_count < MIN_GUESSER_COUNT {
            return Err(PartyError::InvalidMembersRole);
        }

        Ok(Party {
            id: Id::new(),
            members: members.to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{common::player_name::PlayerName, entity::player::Player};

    use super::*;

    #[test]
    fn test_new_party() {
        // 正常系
        let members = vec![
            Member::Questioner(Player::new(PlayerName::new("questioner"))),
            Member::Guesser(Player::new(PlayerName::new("guesser"))),
        ];
        let party = Party::new(&members);
        assert!(party.is_ok());

        // メンバーが10人以上
        let members = vec![
            Member::Questioner(Player::new(PlayerName::new("questioner"))),
            Member::Guesser(Player::new(PlayerName::new("guesser"))),
            Member::Guesser(Player::new(PlayerName::new("guesser"))),
            Member::Guesser(Player::new(PlayerName::new("guesser"))),
            Member::Guesser(Player::new(PlayerName::new("guesser"))),
            Member::Guesser(Player::new(PlayerName::new("guesser"))),
            Member::Guesser(Player::new(PlayerName::new("guesser"))),
            Member::Guesser(Player::new(PlayerName::new("guesser"))),
            Member::Guesser(Player::new(PlayerName::new("guesser"))),
            Member::Guesser(Player::new(PlayerName::new("guesser"))),
            Member::Guesser(Player::new(PlayerName::new("guesser"))),
            Member::Guesser(Player::new(PlayerName::new("guesser"))),
        ];
        let party = Party::new(&members);
        assert!(party.is_err());

        // Questionerが0人
        let members = vec![Member::Guesser(Player::new(PlayerName::new("guesser")))];
        let party = Party::new(&members);
        assert!(party.is_err());

        // guesserが0人
        let members = vec![Member::Questioner(Player::new(PlayerName::new(
            "questioner",
        )))];
        let party = Party::new(&members);
        assert!(party.is_err());

        // Questionerが2人
        let members = vec![
            Member::Questioner(Player::new(PlayerName::new("questioner"))),
            Member::Questioner(Player::new(PlayerName::new("questioner"))),
            Member::Guesser(Player::new(PlayerName::new("guesser"))),
        ];
        let party = Party::new(&members);
        assert!(party.is_err());

        let members = vec![
            Member::Questioner(Player::new(PlayerName::new("questioner"))),
            Member::Questioner(Player::new(PlayerName::new("Questioner"))),
            Member::Guesser(Player::new(PlayerName::new("guesser"))),
        ];
        let party = Party::new(&members);
        assert!(party.is_err());
    }
}
