use serde::{Deserialize, Serialize};





pub struct Conversation {
    pub messages: Vec<Message>,
}



pub struct Message {
    pub user: String,
    pub text: bool,
}