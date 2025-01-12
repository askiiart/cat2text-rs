use crate::core;

pub fn alphabet() -> Vec<String> {
    return vec!["meow", "mrrp", "mreow", "mrow"]
        .into_iter()
        .map(|a| a.to_string())
        .collect();
}

/// How many words long a character is when translated to catspeak
pub fn char_length() -> u32 {
    return 3;
}

pub fn encode(text: String) -> String {
    // makes it lowercase and split by spaces
    let words: Vec<String> = text
        .to_ascii_lowercase()
        .split(" ")
        .map(|item| return item.to_string())
        .collect();
    let mut words_as_bytes: Vec<Vec<u8>> = Vec::new();
    for word in words {
        // convert to bytes then subtract by 96
        words_as_bytes.push(word.as_bytes().into_iter().map(|item| item - 96).collect());
    }

    let mut results: Vec<Vec<String>> = Vec::new();
    for i in 0..words_as_bytes.len() {
        results.push(Vec::new());
        for j in 0..words_as_bytes[i].len() {
            results[i].push(core::num_to_cat(words_as_bytes[i][j] as u32, alphabet(), char_length()));
        }
    }

    let results: Vec<String> = results.into_iter().map(|item| item.join(" ")).collect();
    let results = results.join("; ");
    return results;

}
