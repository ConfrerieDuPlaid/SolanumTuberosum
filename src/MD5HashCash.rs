use crate::structs::Challenge;
use crate::structs::MD5HashCashOutput;
use crate::structs::ChallengeAnswer;
use crate::structs::ChallengeResolve;
use crate::structs::MD5HashCashInput;
use md5::{Md5, Digest};
use hex_literal::hex;

pub struct MD5HashCashResolver {
    pub input: MD5HashCashInput
}

impl ChallengeResolve for MD5HashCashResolver{

    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        "HashCash".to_string()
    }

    fn new(input: Self::Input) -> Self {
        MD5HashCashResolver{
            input
        }
    }

    fn solve(&self) -> Self::Output {
        println!("{} {}", self.input.complexity, self.input.message.len());

        let seed: &str = "0000000000000034C";
        let msg: String = seed.to_owned() + self.input.message.as_str();
        println!("{}", msg);

        let mut hasher = Md5::new();
        hasher.update(msg.into_bytes());
        let result = hasher.finalize();
        println!("{:?}", result.to_vec());

        MD5HashCashOutput{
            seed: 0,
            hashcode: "".to_string(),
        }
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
}