use std::collections::HashMap;

use crate::structs::ChallengeResolve;
use crate::structs::RecoverSecretInput;
use crate::structs::RecoverSecretOutput;

pub struct RecoverSecretResolver {
    pub input: RecoverSecretInput
}

impl ChallengeResolve for RecoverSecretResolver {
    type Input = RecoverSecretInput;
    type Output = RecoverSecretOutput;

    /// Name of the challenge
    fn name() -> String { "RecoverSecret".to_string() }

    /// Create a challenge from the specific input
    fn new(input: Self::Input) -> Self {
        RecoverSecretResolver {
            input
        }
    }

    /// Solve the challenge.
    /// Initialize a vector of strings with the substrings.
    /// For each tuple size, take the first tuple_size letters and put them in the vector.
    /// Resolve the vector of strings.
    /// Return the resolved string.
    fn solve(&self) -> Self::Output {
        let substrings: &mut Vec<String> = &mut vec![];
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

    fn verify(&self, _answer: &Self::Output) -> bool {
        todo!()
    }
}
/// Check if at least one strings is not empty
fn strings_are_not_empty(strings: &Vec<String>) -> bool {
    for str in strings {
        if str.len() > 0 {
            return true;
        }
    }
    return false;
}
/// Find all first letters of all substrings and count how many times they are present
fn find_first_possible_letters<'a> (first_letters: &mut HashMap<&'a str, usize>, substrings: &'a Vec<String>) {
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
/// Set count at 0 for all letters that are not at the beginning of a substring
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
/// Loop until find a letter that is present at least once
fn find_first_valid_letter<'a>(first_letters: &'a mut HashMap<&'a str, usize>) -> &'a str {
    for (l, count) in first_letters {
        if *count != 0 {
            return *l;
        }
    }
    ""
}
/// Remove the letter from all substrings if it begins with it
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

/// Resolve the challenge.
/// It will loop until all substrings are empty.
/// For each loop, it will find all first letters of all substrings and count how many times they are present.
/// Then it will unset all letters that are not at the beginning of a substring.
/// Then it will find the first letter that is present at least once.
/// Then it will remove the letter from all substrings if it begins with it.
/// Finally it will add the letter to the word.
/// It will return the word.
fn resolve (substrings: &mut Vec<String>) -> String {
    let mut word: String = "".to_string();
    while strings_are_not_empty(&substrings) == true {
        let first_letters: &mut HashMap<&str, usize> = &mut HashMap::new() ;
        let substrings_clone = substrings.clone();

        find_first_possible_letters(first_letters, &substrings_clone);
        unset_invalid_first_letters(&substrings_clone, first_letters);

        let l = find_first_valid_letter(first_letters);
        word += &l.to_string();
        remove_first_letter_from_substrings(substrings, l);
    }
    return word;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_remove_first_letter_from_tuple() {
        let substrings: &mut Vec<String> = &mut vec!["aerty".to_string(), "zaoei".to_string(), "zinata".to_string()];
        let expected = &vec!["aerty".to_string(), "aoei".to_string(), "inata".to_string()];
        remove_first_letter_from_substrings(substrings, &"z");
        assert_eq!(expected, substrings);
    }

    #[test]
    fn should_unset_invalid_first_letter() {
        let substrings: &mut Vec<String> = &mut vec!["aerty".to_string(), "zaoei".to_string(), "zinata".to_string()];
        let first_letters: &mut HashMap<&str, usize> = &mut HashMap::from([
            ( "a", 1 ),
            ("z", 2)
        ]) ;
        let expected_first_letters: &mut HashMap<&str, usize> = &mut HashMap::from([
            ( "a", 0 ),
            ("z", 2)
        ]) ;
        unset_invalid_first_letters(substrings, first_letters);
        assert_eq!(expected_first_letters, first_letters);
    }

    #[test]
    fn should_find_valid_first_letter() {
        let first_letters: &mut HashMap<&str, usize> = &mut HashMap::from([
            ( "a", 0 ),
            ("z", 2)
        ]) ;
        let letter = find_first_valid_letter(first_letters);
        assert_eq!("z", letter);
    }

    #[test]
    fn should_find_first_possible_letters() {
        let substrings: &mut Vec<String> = &mut vec!["aerty".to_string(), "zaoei".to_string(), "zinata".to_string()];
        let first_letters: &mut HashMap<&str, usize> = &mut HashMap::new();
        let expected_first_letters: &mut HashMap<&str, usize> = &mut HashMap::from([
            ("z", 2),
            ( "a", 1)
        ]) ;
        find_first_possible_letters(first_letters, substrings);
        assert_eq!(expected_first_letters, first_letters);
    }

    #[test]
    fn strings_should_be_empty() {
        let substrings: &mut Vec<String> = &mut vec!["".to_string(), "".to_string(), "".to_string()];
        let are_not_empty = strings_are_not_empty(substrings);
        assert_eq!(false, are_not_empty);
    }

    #[test]
    fn strings_should_not_be_empty() {
        let substrings: &mut Vec<String> = &mut vec!["".to_string(), "a".to_string(), "".to_string()];
        let are_not_empty = strings_are_not_empty(substrings);
        assert_eq!(true, are_not_empty);
    }
}
