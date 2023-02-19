use std::collections::HashMap;
use std::io::empty;
use std::iter::Map;

use crate::structs::Challenge;
use crate::structs::ChallengeAnswer;
use crate::structs::ChallengeResolve;
use crate::structs::RecoverSecretInput;
use crate::structs::RecoverSecretOutput;

pub struct RecoverSecretResolver {
    pub input: RecoverSecretInput
}

impl ChallengeResolve for RecoverSecretResolver {
    type Input = RecoverSecretInput;
    type Output = RecoverSecretOutput;

    fn name() -> String { "RecoverSecret".to_string() }

    fn new(input: Self::Input) -> Self {
        RecoverSecretResolver {
            input
        }
    }

    fn solve(&self) -> Self::Output {
        let mut substrings: &mut Vec<String> = &mut vec![];
        let mut index = 0;
        for i in 0..self.input.tuple_sizes.len() {
            let tuple_size = self.input.tuple_sizes[i];
            substrings.insert(i, self.input.letters[index .. index+tuple_size].to_string());
            index += tuple_size;
        }
        let word = resolve(substrings);
        return RecoverSecretOutput {
            secret_sentence: word,
        };
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
}

fn strings_are_not_empty(strings: &Vec<String>) -> bool {
    for str in strings {
        if str.len() > 0 {
            return true;
        }
    }
    return false;
}

fn find_fist_possible_letters <'a> (first_letters: &mut HashMap<&'a str, usize>, substrings: &'a Vec<String>) {
    for str in substrings {
        let l = str.get(0..1);
        match l {
            None => {}
            Some(letter) => {
                let mut value = 1;
                if first_letters.contains_key(letter) {
                    let letter_count = first_letters.get_mut(letter);
                    match letter_count {
                        None => {}
                        Some(count) => {
                            value += *count;
                        }
                    }

                    first_letters.remove(letter);
                }
                first_letters.insert(letter, value);
            }
        }
    }
}

fn unset_invalid_first_letters(substrings: &Vec<String>, first_letters: &mut HashMap<&str, usize>) {
    for (letter, count) in first_letters {
        for str in substrings {
            let l_position = str.find(letter);
            match l_position {
                None => {}
                Some(position) => {
                    if position != 0 {
                        *count = 0;
                    }
                }
            }

        }
    }
}

fn find_first_valid_letter<'a>(first_letters: &'a mut HashMap<&'a str, usize>) -> &'a str {
    for (l, count) in first_letters {
        if *count != 0 {
            return *l;
        }
    }
    ""
}

fn remove_first_letter_from_substrings(substrings: &mut Vec<String>, first_letter: &str) {
    let cpy = substrings.clone();
    substrings.clear();
    for i in 0..cpy.len() {
        let position = cpy[i].find(first_letter);
        match position {
            Some(_) => {
                let substr = cpy[i].get(1..cpy[i].len());
                match substr {
                    Some(s) => {
                        substrings.insert(i, s.to_string())
                    }
                    None => {}
                }
            }
            None => {
                substrings.insert(i, cpy[i].to_string())
            }
        }
    }
}

fn resolve (substrings: &mut Vec<String>) -> String {
    let mut word: String = "".to_string();
    while strings_are_not_empty(&substrings) == true {
        let mut first_letters: &mut HashMap<&str, usize> = &mut HashMap::new() ;
        let substrings_clone = substrings.clone();

        find_fist_possible_letters(first_letters, &substrings_clone);
        unset_invalid_first_letters(&substrings_clone, first_letters);

        let l = find_first_valid_letter(first_letters);
        word += &l.to_string();
        remove_first_letter_from_substrings(substrings, l);
    }
    return word;
}