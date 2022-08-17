use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)] //use to send messages over TCP stream
pub struct Message {
    pub id: u32,
    pub room: String,
    pub name: String,
    pub content: String,
    pub timestamp: u64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
