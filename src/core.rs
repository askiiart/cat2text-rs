//! Handles conversion one letter at a time for any base
// based off this SO answer: https://stackoverflow.com/a/1119769
/// Converts a [`u32`] to catspeak
///
/// ```
/// use cat2text::core::{num_to_cat, char_length};
/// use cat2text::base4::alphabet;
///
/// assert_eq!("meow mreow mrrp".to_string(), num_to_cat(9, alphabet(), char_length(4)));
/// ```
pub fn num_to_cat(num: u32, alphabet: Vec<String>, char_length: u32) -> String {
    let mut num: u32 = num.clone();
    let base: u32 = alphabet.len() as u32;

    // base*n*-ifying logic
    let mut nums: Vec<u32> = Vec::new();
    while (nums.len() as u32) < char_length {
        nums.push((num as u32) % base);
        num /= base;
    }
    nums.reverse();

    let mut result: Vec<String> = Vec::new();
    for item in nums {
        result.push(alphabet[item as usize].clone());
    }

    return result.join(" ");
}

/// Converts catspeak to a [`u32`]
///
/// ```
/// use cat2text::core::{cat_to_num, bytes::char_length};
/// use cat2text::base4::alphabet;
///
/// let text = vec!["meow".to_string(), "mrrp".to_string(), "mrow".to_string(), "meow".to_string()];
///
/// assert_eq!(28, cat_to_num(text, alphabet(), char_length(4)));
/// ```
pub fn cat_to_num(text: Vec<String>, alphabet: Vec<String>, char_length: u32) -> u32 {
    let mut nums: Vec<u32> = Vec::new();
    for word in text {
        for i in 0..alphabet.len() {
            if word == alphabet[i] {
                nums.push(i as u32);
                break;
            }
        }
    }

    let base = alphabet.len();
    let mut num = 0 as u32;
    for n in 0..nums.len() {
        let power = (char_length - (n as u32 + 1)) as u32;
        num += nums[n] * (base as u32).pow(power);
    }
    return num;
}

/// Splits a word encoded in catspeak every *x* segments
///
/// Used for decoding by splitting words apart into letters which can then be decoded individually
///
/// ```ignore
/// use cat2text::core::split_every_x;
///
/// assert_eq!(vec!["meow meow mrrp".to_string(), "meow mreow mrrp".to_string()], split_every_x("meow meow mrrp meow mreow mrrp".to_string(), 3));
/// ```
pub(crate) fn split_every_x(text: impl AsRef<str>, x: u32) -> Vec<String> {
    let x = x as usize;
    let delim = " ";
    let tmp: Vec<String> = text
        .as_ref()
        .split(delim)
        .map(|item| item.to_string())
        .collect();
    let mut output: Vec<String> = Vec::new();
    for i in 0..tmp.len() {
        if i % x == 0 {
            output.push(String::new())
        }
        output[i / x] += tmp[i].as_str();
        output[i / x] += " ";
    }

    // trim everything before sending it back
    output = output
        .into_iter()
        .map(|item| item.trim().to_string())
        .collect();
    return output;
}

/// Returns all cat sounds in the catspeak alphabet
///
/// ```
/// use cat2text::core::alphabet;
///
/// println!("{:?}", alphabet());
/// ```
pub fn alphabet() -> Vec<String> {
    return vec![
        "meow", "mrrp", "mreow", "mrow", "nya~", "nyaaaa~", "mraow", "mew", "prrp", "mewo",
        "purrrr", "nya", "miao", "miau", "miauw", "mrow~",
    ]
    .into_iter()
    .map(|a| a.to_string())
    .collect();
}

/// Returns the max base that can be used
///
/// For example, if the available alphabet was `["meow", "mrrp", "mreow", "mrow"]`, the max base would be 4
///
/// ```
/// use cat2text::core::max_base;
///
/// println!("{}", max_base());
/// ```
pub fn max_base() -> u32 {
    return alphabet().len() as u32;
}

/// Returns the minimum catspeak words per character needed for this base
///
/// ```
/// use cat2text::core::char_length;
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

pub mod bytes {
    /// Returns the minimum catspeak words per character needed for this base for bytes
    ///
    /// ```
    /// use cat2text::core::bytes::char_length;
    ///
    /// let base = 16;
    /// assert_eq!(char_length(base), 2)
    /// ```
    pub fn char_length(base: u32) -> u32 {
        for i in 1..base + 1 {
            let num = base.pow(i);
            if num > 255 {
                return i;
            }
        }
        return u32::MAX;
    }
}
