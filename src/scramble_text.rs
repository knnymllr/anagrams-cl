use rand::seq::SliceRandom;
use rand::{rng};

pub fn scramble_text(input: &str) -> String {
    let mut rng = rng();
    let mut letters: Vec<char> = input
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    letters.shuffle(&mut rng);

    let mut result = String::new();
    let mut letter_iter = letters.into_iter();

    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            result.push(letter_iter.next().unwrap());
        } else {
            result.push(c); // Preserve punctuation and spaces
        }
    }

    result
}
