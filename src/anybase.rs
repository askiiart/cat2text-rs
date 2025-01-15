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
/// 
/// ```
/// use cat2text::anybase::char_length;
/// 
/// let base = 10;
/// assert_eq!(char_length(base), 2)
/// ```
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
///
/// let text = "i love cats".to_string();
/// let base = 10;
/// let char_length = anybase::char_length(base);
///
/// assert_eq!("meow mewo; mrrp mreow mrrp nyaaaa~ mreow mreow meow nyaaaa~; meow mrow meow mrrp mreow meow mrrp mewo", anybase::encode(text, base, char_length));
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
///
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
        for engl_letter in core::split_every_x(engl_word, char_length) {
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

pub mod bytes {
    use super::alphabet;
    use crate::core;
    /// Encodes from bytes into catspeak
    ///
    /// ```
    /// use cat2text::anybase::{bytes::encode, char_length};
    ///
    /// let bytes = &[9, 1];
    /// let base = 10;
    /// let char_length = char_length(base);
    ///
    /// assert_eq!("meow mewo meow mrrp", encode(bytes, base, char_length));
    /// ```
    pub fn encode(bytes: impl AsRef<[u8]>, base: u32, char_length: u32) -> String {
        let mut output = String::new();
        let mut shortened_alphabet = alphabet();
        shortened_alphabet.truncate(base as usize);
        for byte in bytes.as_ref() {
            output += core::num_to_cat(*byte as u32, shortened_alphabet.clone(), char_length).as_str();
            output += " ";
        }
        return output.trim().to_string();
    }

    /// Decodes catspeak into bytes
    ///
    /// ```
    /// use cat2text::anybase::{bytes::decode, char_length};
    ///
    /// let text = "mreow mrrp meow mrrp".to_string();
    /// let base = 10;
    /// let char_length = char_length(base);
    ///
    /// assert_eq!(
    ///     vec![21, 1],
    ///     decode(text, base, char_length)
    /// );
    /// ```
    pub fn decode(text: String, base: u32, char_length: u32) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::new();
        let mut shortened_alphabet = alphabet();
        shortened_alphabet.truncate(base as usize);
        for byte in core::split_every_x(text.clone(), char_length) {
            output.push(core::cat_to_num(
                byte.split(" ").map(|item| item.to_string()).collect(),
                shortened_alphabet.clone(),
                char_length,
            ) as u8);
        }
        return output.into();
    }
}
