//! Handles conversion one word at a time for any base
// based off this SO answer: https://stackoverflow.com/a/1119769
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
