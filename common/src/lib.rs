use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)] //use to send messages over TCP stream
pub struct Message {
    pub id: u32,
    pub room: String,
    pub name: String,
    pub content: String,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)] //use to send messages over TCP stream
pub struct UserJoinOrLeaveRoom {//used when users enter/leave rooms
    pub id: u32,
    pub room: String,
    pub username: String,
    pub content: String,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)] //use to send messages over TCP stream
pub struct Ping {//any other sort of message
    pub id: u32,
    pub room: String,
    pub content: String,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)] //use to send messages over TCP stream
pub enum NetworkMessage {
    Message(Message),
    UserJoinOrLeaveRoom (UserJoinOrLeaveRoom),
    Ping(Ping)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
