use std::sync::Arc;

use crate::entity::room::repository::RoomRepository;

pub struct RoomService<R>
where
    R: RoomRepository,
{
    repository: Arc<R>,
}

impl<R> RoomService<R>
where
    R: RoomRepository,
{
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
    
}
