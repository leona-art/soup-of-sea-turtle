use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use domain::entity::{
    game::{repository::GameRepository, Game},
    log::GameLog,
    room::{repository::RoomRepository, Room},
    user::{repository::UserRepository, User},
};

#[derive(Debug)]
pub struct InMemoryUserRepository {
    users: Mutex<HashMap<String, User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        InMemoryUserRepository {
            users: Mutex::new(HashMap::new()),
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    fn save(&self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        let mut users = self.users.lock().unwrap();
        users.insert(user.id(), user);
        Ok(())
    }

    fn find_by_id(&self, id: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let users = self.users.lock().unwrap();
        Ok(users.get(id).cloned())
    }
}

#[derive(Debug)]
pub struct InMemoryRoomRepository {
    rooms: Mutex<HashMap<String, Room>>,
}
impl InMemoryRoomRepository {
    pub fn new() -> Self {
        InMemoryRoomRepository {
            rooms: Mutex::new(HashMap::new()),
        }
    }
}

impl RoomRepository for InMemoryRoomRepository {
    fn save(&self, room: Room) -> Result<(), Box<dyn std::error::Error>> {
        let mut rooms = self.rooms.lock().unwrap();
        rooms.insert(room.id(), room);
        Ok(())
    }

    fn find_by_id(&self, id: &str) -> Result<Option<Room>, Box<dyn std::error::Error>> {
        let rooms = self.rooms.lock().unwrap();
        Ok(rooms.get(id).cloned())
    }
    
    fn find_all(&self) -> Result<Vec<Room>, Box<dyn std::error::Error>> {
        let rooms = self.rooms.lock().unwrap();
        Ok(rooms.values().cloned().collect())
    }
    
}

#[derive(Debug)]
pub struct InMemoryGameRepository {
    games: Mutex<HashMap<String, Game>>,
}

impl InMemoryGameRepository {
    pub fn new() -> Self {
        InMemoryGameRepository {
            games: Mutex::new(HashMap::new()),
        }
    }
}

impl GameRepository for InMemoryGameRepository {
    fn save(&self, game: Game) -> Result<(), Box<dyn std::error::Error>> {
        let mut games = self.games.lock().unwrap();
        games.insert(game.id(), game);
        Ok(())
    }

    fn find_by_id(&self, id: &str) -> Result<Option<Game>, Box<dyn std::error::Error>> {
        let games = self.games.lock().unwrap();
        Ok(games.get(id).cloned())
    }
}
#[derive(Debug)]
pub struct InMemoryLogRepository {
    logs: Mutex<HashMap<String, GameLog>>,
}
impl InMemoryLogRepository {
    pub fn new() -> Self {
        InMemoryLogRepository {
            logs: Mutex::new(HashMap::new()),
        }
    }
}

impl InMemoryLogRepository {
    pub fn save(&self, log: GameLog) -> Result<(), Box<dyn std::error::Error>> {
        let mut logs = self.logs.lock().unwrap();
        logs.insert(log.id().to_string(), log);
        Ok(())
    }

    pub fn find_by_id(&self, id: &str) -> Result<Option<GameLog>, Box<dyn std::error::Error>> {
        let logs = self.logs.lock().unwrap();
        Ok(logs.get(id).cloned())
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_user_repository(){
        let repo=InMemoryUserRepository::new();
        let user=User::new("test").unwrap();
        repo.save(user.clone()).unwrap();
        let found=repo.find_by_id(&user.id().to_string()).unwrap().unwrap();
        assert_eq!(found,user);
    }

    #[test]
    fn test_room_repository(){
        let repo=InMemoryRoomRepository::new();
        let room=Room::new("test").unwrap();
        repo.save(room.clone()).unwrap();
        let found=repo.find_by_id(&room.id()).unwrap().unwrap();
        assert_eq!(found,room);
    }

    #[test]
    fn test_game_repository(){
        let repo=InMemoryGameRepository::new();
        let users=vec![User::new("test1").unwrap(),User::new("test2").unwrap()];
        let game=Game::new(&users);
        repo.save(game.clone()).unwrap();
        let found=repo.find_by_id(&game.id()).unwrap().unwrap();
        assert_eq!(found,game);
    }

    #[test]
    fn test_log_repository(){
        let repo=InMemoryLogRepository::new();
        let log=GameLog::new();
        repo.save(log.clone()).unwrap();
        let found=repo.find_by_id(&log.id()).unwrap().unwrap();
        assert_eq!(found,log);
    }

    #[test]
    fn multithread_test(){
        let user_repo=Arc::new(InMemoryUserRepository::new());

        let mut handle=Vec::new();

        for i in 0..10{
            let repo=user_repo.clone();
            handle.push(std::thread::spawn(move ||{
                let user=User::new(&format!("test{}",i)).unwrap();
                repo.save(user).unwrap();
            }));
        }

        for h in handle{
            h.join().unwrap();
        }
        
        let users=user_repo.users.lock().unwrap();
        assert_eq!(users.len(),10);
    }
}