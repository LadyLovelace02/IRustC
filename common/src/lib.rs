#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub id: u32,
    pub room: String,
    pub name: String,
    pub content: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Room {
    pub id: u32,
    pub name: String,
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct User {
    pub id: u32,
    pub name: String,
    pub rooms: Vec<Room>,
}

#[cfg(test)]
mod tests {
    use crate::{Room, User};

    #[test]
    fn test_10000_rooms() {
        let mut rooms = Vec::new();
        for _ in 0..10000 {
            rooms.push(Room {
                id: 0,
                name: "".to_string(),
                messages: Vec::new(),
            });
        }
    }

    #[test]
    fn test_many_users_in_rooms_sending_messages() {
        let mut users = Vec::new();
        for _ in 0..100 {
            users.push(User {
                id: 0,
                name: "".to_string(),
                rooms: Vec::new(),
            });
        }

        let mut rooms = Vec::new();
        for _ in 0..100 {
            rooms.push(Room {
                id: 0,
                name: "".to_string(),
                messages: Vec::new(),
            });
        }

        for user in users.iter_mut() {
            for room in rooms.iter() {
                user.rooms.push(room.clone());
            }
        }

        users.iter_mut().for_each(|user| {
            for room in rooms.iter() {
                user.rooms.push(room.clone());
            }
        });
    }
}
