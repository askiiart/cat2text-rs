//! Handles conversion one letter at a time for any base
// based off this SO answer: https://stackoverflow.com/a/1119769
/// Converts a [`u32`] to catspeak
/// 
/// ```
/// use cat2text::core::num_to_cat;
/// use cat2text::base4::{alphabet, char_length};
/// 
/// assert_eq!("meow mreow mrrp".to_string(), num_to_cat(9, alphabet(), char_length()));
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
/// use cat2text::core::cat_to_num;
/// use cat2text::base4::{alphabet, char_length};
/// 
/// let letter = vec!["meow".to_string(), "mreow".to_string(), "mrrp".to_string()];
/// 
/// assert_eq!(9, cat_to_num(letter, alphabet(), char_length()));
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

/// Splits a cat word into every 3 segments
/// 
/// ```
/// use cat2text::core::split_every_3;
/// 
/// assert_eq!(vec!["meow meow mrrp".to_string(), "meow mreow mrrp".to_string(), "mreow meow mrrp".to_string()], split_every_3("meow meow mrrp meow mreow mrrp mreow meow mrrp".to_string()));
/// ```
pub fn split_every_3(text: String) -> Vec<String> {
    let delim = " ";
    let tmp: Vec<String> = text.split(delim).map(|item| item.to_string()).collect();
    let mut output: Vec<String> = Vec::new();
    for i in 0..tmp.len() {
        if i % 3 == 0 {
            output.push(String::new())
        }
        output[i / 3] += tmp[i].as_str();
        if i % 3 != 2 {
            output[i / 3] += " ";
        }
    }
    return output;
}