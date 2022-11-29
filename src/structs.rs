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
    MD5HashCash (MD5HashCashInput)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MD5HashCashInput{
    pub complexity: u8,
    pub message: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashOutput {
    pub seed: u64,
    pub hashcode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput)
}

#[derive(Serialize, Deserialize, Debug)]
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
    Challenge(Challenge),
    ChallengeResult{
        answer: ChallengeAnswer,
        next_target: String
    }
}

pub trait ChallengeResolve{
    /// Données en entrée du challenge
    type Input;
    /// Données en sortie du challenge
    type Output;
    /// Nom du challenge
    fn name() -> String;
    /// Create a challenge from the specific input
    fn new(input: Self::Input) -> Self;
    /// Résout le challenge
    fn solve(&self) -> Self::Output;
    /// Vérifie qu'une sortie est valide pour le challenge
    fn verify(&self, answer: &Self::Output) -> bool;
}