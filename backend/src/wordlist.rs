use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use rand::{self, Rng};

pub struct Wordlist {
    words: Vec<String>,
}

impl Wordlist {
    pub fn new() -> Wordlist {
        let mut words = vec![];
        let f = File::open("../words/saoldict.txt").unwrap();
        let file = BufReader::new(&f);
        for word in file.lines() {
            let word = word.unwrap()
                .replace("+", "")
                .replace("=", "")
                .split("|")
                .next()
                .unwrap()
                .to_owned();
            words.push(word);
        }
        Wordlist {
            words,
        }
    }

    pub fn get_random_word(&self) -> String {
        let mut rng = rand::thread_rng();
        let i = rng.gen::<usize>() % self.words.len();
        self.words[i].clone()
    }

    pub fn has_word(&self, word: &String) -> bool {
        self.words.contains(word)
    }
}
