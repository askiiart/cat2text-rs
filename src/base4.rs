use crate::core;
use crate::anybase;

pub fn alphabet() -> Vec<String> {
    let mut tmp = core::alphabet();
    tmp.truncate(4);
    return tmp;
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
    return anybase::encode(text.as_ref().to_string(), 4, char_length());
}

/// Decodes base 4 catspeak to english text
///
/// ```
/// use cat2text::base4::decode;
///
/// assert_eq!("i love cats", decode("meow mreow mrrp; meow mrow meow meow mrow mrow mrrp mrrp mreow meow mrrp mrrp; meow meow mrow meow meow mrrp mrrp mrrp meow mrrp meow mrow".to_string()));
/// ```
pub fn decode(text: String) -> String {
    return anybase::decode(text, 4, char_length())
}

pub mod bytes {
    use crate::anybase;
    use super::char_length;
    /// Encodes from bytes into catspeak
    ///
    /// ```
    /// use cat2text::base4::bytes::encode;
    ///
    /// assert_eq!("meow mreow mrrp meow meow mrrp", encode(&[9, 1]));
    /// ```
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        anybase::bytes::encode(bytes, 4, char_length())
    }

    /// Decodes catspeak into bytes
    ///
    /// ```
    /// use cat2text::base4::bytes::decode;
    ///
    /// assert_eq!(vec![9, 1], decode("meow mreow mrrp meow meow mrrp".to_string()));
    /// ```
    pub fn decode(text: String) -> Vec<u8> {
        anybase::bytes::decode(text, 4, char_length())
    }
}