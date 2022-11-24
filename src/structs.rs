use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SubscribeResult {
    Ok,
    Err (SubscribeError),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicPlayer{
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Challenge {
    MD5HashCash {
        complexity: u8,
        message: String,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    Hello,
    Welcome {
        version: u8,
    },
    Subscribe {
        name: String,
    },
    SubscribeResult (SubscribeResult),
    PublicLeaderBoard (Vec<PublicPlayer>),
    Challenge(Challenge)
}