use std::ops::Deref;

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

/// Encodes english text into base 4 catspeak
/// 
/// ```
/// use cat2text::base4::encode;
/// 
/// assert_eq!("meow mreow mrrp; meow mrow meow meow mrow mrow mrrp mrrp mreow meow mrrp mrrp; meow meow mrow meow meow mrrp mrrp mrrp meow mrrp meow mrow", encode("i love cats".to_string()))
/// ```
pub fn encode(text: impl AsRef<str>) -> String {
    let text = text.as_ref();
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
            results[i].push(core::num_to_cat(
                words_as_bytes[i][j] as u32,
                alphabet(),
                char_length(),
            ));
        }
    }

    let results: Vec<String> = results.into_iter().map(|item| item.join(" ")).collect();
    let results = results.join("; ");
    return results;
}

/// Decodes base 4 catspeak to english text
///
/// ```
/// use cat2text::base4::decode;
///
/// assert_eq!("i love cats", decode("meow mreow mrrp; meow mrow meow meow mrow mrow mrrp mrrp mreow meow mrrp mrrp; meow meow mrow meow meow mrrp mrrp mrrp meow mrrp meow mrow".to_string()));
/// ```
pub fn decode(text: String) -> String {
    let catspeak_words: Vec<String> = text
        .split("; ")
        .map(|item| item.to_string())
        .collect();
    let mut output: String = String::new();
    for engl_word in catspeak_words {
        let mut word = String::new();
        for engl_letter in core::split_every_x(engl_word, 3) {
            let char_num = core::cat_to_num(
                engl_letter
                    .split(" ")
                    .map(|item| item.to_string())
                    .collect(),
                alphabet(),
                char_length(),
            );
            word += String::from_utf8(vec![(char_num + 96) as u8])
                .unwrap()
                .as_str();
        }
        word += " ";
        output += word.as_str();
    }

    return output.trim().to_string();
}


pub mod bytes {
    use crate::core;
    use crate::base4::alphabet;
/// Encodes from bytes into catspeak
/// 
/// ```
/// use cat2text::base4::bytes::encode;
/// 
/// assert_eq!("meow meow mreow mrrp meow meow meow mrrp", encode(&[9, 1]));
/// ```
pub fn encode(bytes: impl AsRef<[u8]>) -> String {
    let mut output = String::new();
    for byte in bytes.as_ref() {
        output += core::num_to_cat(*byte as u32, alphabet(), 4).as_str();
        output += " ";
    }
    return output.trim().to_string();
}

/// Decodes catspeak into bytes
///
/// ```
/// use cat2text::base4::bytes::decode;
///
/// assert_eq!(vec![9, 1], decode("meow meow mreow mrrp meow meow meow mrrp".to_string()));
/// ```
pub fn decode(text: String) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    for byte in core::split_every_x(text.clone(), 4) {
        output.push(core::cat_to_num(
            byte.split(" ").map(|item| item.to_string()).collect(),
            alphabet(),
            4,
        ) as u8);
    }
    return output;
}

}