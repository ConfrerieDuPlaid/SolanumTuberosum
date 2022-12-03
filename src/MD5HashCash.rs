use crate::structs::Challenge;
use crate::structs::MD5HashCashOutput;
use crate::structs::ChallengeAnswer;
use crate::structs::ChallengeResolve;
use crate::structs::MD5HashCashInput;
use md5::{Md5, Digest};
use hex::encode;
use md5::digest::consts::U64;

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
        let mut seedConuter = 0;
        loop {
            let mut seedConuterHexa = format!("{:x}", seedConuter);
            if seedConuterHexa.len() < 2{
                seedConuterHexa = "0".to_string() + &*format!("{:x}", seedConuter);
            }else{
                seedConuterHexa = format!("{:x}", seedConuter);
            }

            let mut seed: String = "".to_string();
            for zero in 0 .. (16 - seedConuterHexa.len()){
                seed = seed + "0";
            }

            seed += seedConuterHexa.as_str();
            seedConuter += 1;
            let msg: String = seed + self.input.message.as_str();

            let mut hasher = Md5::new();
            hasher.update(msg.into_bytes());
            let result = hasher.finalize();

            let mut hashCode: String = "".to_string();

            for msd5Hash in result.to_vec() {
                let format = format!("{:x}", msd5Hash);
                if format.len() < 2{
                    hashCode = hashCode + "0" + format.as_str();
                }else{
                    hashCode = hashCode + format.as_str();
                }
            }

            let isGood = verifyComplexity(result.to_vec(), self.input.complexity) ;
            if isGood {
                return MD5HashCashOutput{
                    seed: seedConuter,
                    hashcode: hashCode,
                }
            }
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