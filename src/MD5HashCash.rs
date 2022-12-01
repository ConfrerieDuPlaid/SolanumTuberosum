use crate::structs::Challenge;
use crate::structs::MD5HashCashOutput;
use crate::structs::ChallengeAnswer;
use crate::structs::ChallengeResolve;
use crate::structs::MD5HashCashInput;
use md5::{Md5, Digest};
use hex::encode;

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
        let seed: &str = "000000000000034C";
        let msg: String = seed.to_owned() + self.input.message.as_str();

        let mut hasher = Md5::new();
        hasher.update(msg.into_bytes());
        let result = hasher.finalize();

        println!("result = {:?}", result.to_vec());
        let mut hashCode: String = "".to_string();

        for msd5Hash in result.to_vec() {
            hashCode = hashCode + format!("{:x}", msd5Hash).as_str();
            //TODO hashCode implement 0
        }

        println!("hashCode = {}", hashCode);
        let isGood = verifyComplexity(result.to_vec(), 9) ;
        println!("isGood = {}", isGood);
        MD5HashCashOutput{
            seed: 0,
            hashcode: "".to_string(),
        }
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }


}

fn verifyComplexity(hashArray: Vec<u8>, complexity: u32) -> bool{
    let indexMin: u32 = complexity / 8;
    let valueMax: u32 = 2u32.pow(8 - ( complexity % 8 ) );

    for i in 0..indexMin {
        if hashArray.get(i as usize).unwrap() != &0 {
            return false
        }
    };

    hashArray.get(indexMin as usize).unwrap() < &(valueMax as u8)
}