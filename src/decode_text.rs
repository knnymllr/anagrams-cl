pub fn decode_text(input: &str) -> Vec<String> {
    const WORDS: &[u8] = include_bytes!("./files/words.txt");

    let word_str = std::str::from_utf8(WORDS)
    .expect("Invalid UTF-8");

    let words: Vec<&str> = word_str
        .lines()
        .collect();
    
    let lowercase: Vec<String> = word_str
    .lines()
    .map(|line| line.trim().to_lowercase())
    .collect();

    let input_sorted: Vec<char> = {
        let mut chars: Vec<char> = input.chars().collect();
        chars.sort_unstable();
        chars
    };

    let mut matches = Vec::new();

    for (i, word) in lowercase.iter().enumerate() {
        if word.len() != input.len() {
            continue;
        }

        let mut word_chars: Vec<char> = word.chars().collect();
        word_chars.sort_unstable();

        if word_chars == input_sorted {
            matches.push(words[i].to_string());
        }
    }

    matches
}
    