use crate::common::id::Id;

use super::Party;

pub trait PartyRepository {
    fn get_party(id:Id) -> Option<Party>;
    fn save_party(party:Party) -> Result<(), String>;
    fn delete_party(id:Id) -> Result<(), String>;
}