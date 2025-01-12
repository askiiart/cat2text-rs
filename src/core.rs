//! Handles conversion one word at a time for any base

use std::ops::Index;

use crate::base4::alphabet;

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

    let mut result = "".to_string();
    for i in nums.clone() {
        result += alphabet[i as usize].as_str();
        // put a space in between if it's not the last one
        if i != nums.len() as u32 - 1 {
            result += " "
        }
    }

    return result;
}

/// Converts catspeak to a [`u32`]
pub fn cat_to_num(text: Vec<String>, alphabet: Vec<String>, char_length: u32) -> u32 {
    let mut nums: Vec<u32> = Vec::new();
    for word in text.clone() {
        for i in 0..alphabet.len() {
            if word == alphabet[i] {
                nums.push(i as u32);
            }
        }
    }

    let base = alphabet.len();
    let idx = 0;
    let mut num = 0 as u32;
    for n in nums {
        let power = (text.len() - (idx + 1)) as u32;
        num += n * (base as u32).pow(power);
    }
    return num;
}
