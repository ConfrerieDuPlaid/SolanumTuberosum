use md5::{Digest, Md5, Md5Core};
use md5::digest::core_api::CoreWrapper;
use md5::digest::Output;

use crate::structs::ChallengeResolve;
use crate::structs::MD5HashCashInput;
use crate::structs::MD5HashCashOutput;

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
        let mut seed_counter = 0;
        let mut seed_counter_hexa: String;
        let mut hash_code: String;
        let mut seed: String;
        let mut msg: String;
        let mut result: Output<CoreWrapper<Md5Core>>;
        let mut format: String;
        let mut is_valid_complexity: bool;
        loop {
            seed_counter_hexa = format!("{:x}", seed_counter).to_uppercase();
            if seed_counter_hexa.len() < 2{
                seed_counter_hexa = "0".to_string() + seed_counter_hexa.as_str();
            }

            seed = "".to_string();
            for _ in 0 .. (16 - seed_counter_hexa.len()){
                seed = seed + "0";
            }

            seed += seed_counter_hexa.as_str();
            msg = seed.clone() + self.input.message.as_str();

            let mut hasher = Md5::new();
            hasher.update(msg.clone().into_bytes());
            result = hasher.finalize();

            hash_code = "".to_string();

            for msd5_hash in result.to_vec() {
                format = format!("{:x}", msd5_hash);
                if format.len() < 2{
                    hash_code = hash_code + "0" + format.as_str();
                }else{
                    hash_code = hash_code + format.as_str();
                }
            }
            hash_code = hash_code.to_uppercase();
            is_valid_complexity = verify_complexity(result.to_vec(), self.input.complexity) ;

            if is_valid_complexity {
                return MD5HashCashOutput{
                    seed: seed_counter,
                    hashcode: hash_code,
                }
            }
            seed_counter += 1;
        }
    }

    fn verify(&self, _answer: &Self::Output) -> bool {
        todo!()
    }


}

fn verify_complexity(hash_array: Vec<u8>, complexity: u32) -> bool{
    let index_min: u32 = complexity / 8;
    let value_max: u32 = 2u32.pow(8 - ( complexity % 8 ) );

    for i in 0..index_min {
        let i_value = hash_array.get(i as usize);
        match i_value {
            None => {}
            Some(value) => {
                if value != &0 {
                    println!("returning false on {}", *value);
                    return false;
                }
            }
        }
    };

    let value_at_index_min  = hash_array.get(index_min as usize);
    let mut res = false;
    match value_at_index_min {
        None => {}
        Some(value) => {
            res = value < &(value_max as u8);
        }
    }
    return res;
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn verify_valid_complexity() {
        let complexity: u32 = 9;
        let mut hash : Vec<u8> = vec![];
        hash.push(0);
        hash.push(127);
        let verify = verify_complexity(hash, complexity);
        assert_eq!(true, verify);
    }

    #[test]
    fn verify_invalid_complexity() {
        let complexity: u32 = 9;
        let mut hash : Vec<u8> = vec![];
        hash.push(0);
        hash.push(128);
        let verify = verify_complexity(hash, complexity);
        assert_eq!(false, verify);
    }
}
