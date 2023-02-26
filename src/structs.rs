use serde::{Deserialize, Serialize};

pub type PublicLeaderBoard = Vec<PublicPlayer>;

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
    pub name: String,
    pub(crate) stream_id: String,
    pub(crate) score: i32,
    pub(crate) steps: u32,
    pub(crate) is_active: bool,
    pub(crate) total_used_time: f64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Challenge {
    MD5HashCash (MD5HashCashInput),
    RecoverSecret (RecoverSecretInput)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MD5HashCashInput{
    pub complexity: u32,
    pub message: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashOutput {
    pub seed: u64,
    pub hashcode: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecoverSecretInput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecoverSecretOutput {
    pub secret_sentence: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput),
    RecoverSecret(RecoverSecretOutput)
}
#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeValue{
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportedChallengeResult{
    name: String,
    value: ChallengeValue
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
    },
    ChallengeTimeout(
        String
    ),
    RoundSummary{
        challenge: String,
        chain: Vec<ReportedChallengeResult>
    },
    EndOfGame{leader_board: Vec<PublicPlayer>}
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