use serde::{Deserialize, Serialize};




#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conversation {
    pub messages: Vec<Message>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub user: String,
    pub text: bool,
}