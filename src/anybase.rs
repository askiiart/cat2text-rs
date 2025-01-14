use crate::core;

pub fn alphabet() -> Vec<String> {
    return vec![
        "meow", "mrrp", "mreow", "mrow", "nya~", "nyaaaa~", "mraow", "mew", "prrp", "mewo",
        "purrrr", "nya",
    ]
    .into_iter()
    .map(|a| a.to_string())
    .collect();
}

pub fn max_base() -> u32 {
    return alphabet().len() as u32;
}

/// Returns the minimum catspeak words per character needed for this base
pub fn char_length(base: u32) -> u32 {
    for i in 1..base + 1 {
        let num = base.pow(i);
        if num > 26 {
            return i;
        }
    }
    return u32::MAX;
}

/// Encodes text into catspeak using any base up to [`max_base()`]
///
/// `char_length` is set manually, but the minimum can be generated using [`char_length()`]
///
/// ```
/// use cat2text::anybase;
/// let text = "i love cats".to_string();
/// let base = 10;
/// let char_length = anybase::char_length(base);
///
/// assert_eq!("", anybase::encode(text, base, char_length));
/// ```
pub fn encode(text: String, base: u32, char_length: u32) -> String {
    let mut shortened_alphabet = alphabet();
    shortened_alphabet.truncate(base as usize);

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
                shortened_alphabet.clone(),
                char_length,
            ));
        }
    }

    let results: Vec<String> = results.into_iter().map(|item| item.join(" ")).collect();
    let results = results.join("; ");
    return results;
}

/// Decodes catspeak into text using any base up to [`max_base()`]
///
/// `char_length` is set manually, but the minimum can be generated using [`char_length()`]
///
/// ```
/// use cat2text::anybase;
/// let text = "meow mewo; mrrp mreow mrrp nyaaaa~ mreow mreow meow nyaaaa~; meow mrow meow mrrp mreow meow mrrp mewo".to_string();
/// let base = 10;
/// let char_length = anybase::char_length(base);

/// assert_eq!("i love cats", anybase::decode(text, base, char_length));
/// ```
pub fn decode(text: String, base: u32, char_length: u32) -> String {
    let catspeak_words: Vec<String> = text.split("; ").map(|item| item.to_string()).collect();
    let mut output: String = String::new();
    let mut shortened_alphabet = alphabet();
    shortened_alphabet.truncate(base as usize);
    for engl_word in catspeak_words {
        let mut word = String::new();
        for engl_letter in core::split_every_x(engl_word, char_length as usize) {
            let char_num = core::cat_to_num(
                engl_letter
                    .split(" ")
                    .map(|item| item.to_string())
                    .collect(),
                shortened_alphabet.clone(),
                char_length,
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

