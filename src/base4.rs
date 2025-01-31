//! This module handles base 4, like the original [Cat2Text](https://github.com/Evelyn3440/Cat2Text); it can translate either english text a-z, or byte arrays (see [`bytes`])
use crate::anybase;
use crate::core;

/// Returns the alphabet used by `cat2text::base4`
pub fn alphabet() -> Vec<String> {
    let mut tmp = core::alphabet();
    tmp.truncate(4);
    return tmp;
}

/// Encodes english text into base 4 catspeak
///
/// ```
/// use cat2text::base4::encode;
///
/// assert_eq!("meow mreow mrrp; meow mrow meow meow mrow mrow mrrp mrrp mreow meow mrrp mrrp; meow meow mrow meow meow mrrp mrrp mrrp meow mrrp meow mrow", encode("i love cats".to_string()))
/// ```
pub fn encode(text: impl AsRef<str>) -> String {
    return anybase::encode(text.as_ref().to_string(), 4, core::char_length(4));
}

/// Decodes base 4 catspeak to english text
///
/// ```
/// use cat2text::base4::decode;
///
/// assert_eq!("i love cats", decode("meow mreow mrrp; meow mrow meow meow mrow mrow mrrp mrrp mreow meow mrrp mrrp; meow meow mrow meow meow mrrp mrrp mrrp meow mrrp meow mrow".to_string()));
/// ```
pub fn decode(text: impl AsRef<str>) -> String {
    return anybase::decode(text, 4, core::char_length(4));
}

pub mod bytes {
    //! This handles encoding and decoding bytes to/from catspeak
    use crate::anybase;
    use crate::core::bytes::char_length;
    /// Encodes from bytes into catspeak
    ///
    /// ```
    /// use cat2text::base4::bytes::encode;
    ///
    /// assert_eq!("mrow mrow mrow mrrp meow meow meow mrrp", encode(&[253, 1]));
    /// ```
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        anybase::bytes::encode(bytes, 4, char_length(4))
    }

    /// Decodes catspeak into bytes
    ///
    /// ```
    /// use cat2text::base4::bytes::decode;
    ///
    /// assert_eq!(vec![253, 1], decode("mrow mrow mrow mrrp meow meow meow mrrp".to_string()));
    /// ```
    pub fn decode(text: impl AsRef<str>) -> Vec<u8> {
        anybase::bytes::decode(text, 4, char_length(4))
    }
}
