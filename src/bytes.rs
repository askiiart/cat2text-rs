use crate::base4::alphabet;
use crate::core;

/// Encodes from bytes into catspeak
/// 
/// ```
/// use cat2text::bytes::from_bytes;
/// 
/// assert_eq!("meow meow mreow mrrp meow meow meow mrrp", from_bytes(vec![9, 1]));
/// ```
pub fn from_bytes(bytes: Vec<u8>) -> String {
    let mut output = "".to_string();
    for byte in bytes {
        output += core::num_to_cat(byte as u32, alphabet(), 4).as_str();
        output += " ";
    }
    return output.trim().to_string();
}

/// Decodes catspeak into bytes
///
/// ```
/// use cat2text::bytes::to_bytes;
///
/// assert_eq!(vec![9, 1], to_bytes("meow meow mreow mrrp meow meow meow mrrp".to_string()));
/// ```
pub fn to_bytes(text: String) -> Vec<u8> {
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
