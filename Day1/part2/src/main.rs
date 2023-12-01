use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let lines = read_lines("input");
    let mut number_vectors:Vec<u32> = Vec::new();
    let mut totalsum:u32 = 0;
    for line in lines.iter() {
        let digits = get_digits_in_string(line);
        number_vectors.push(add_first_and_last_digit_from_vector(&digits));
    }
    for sum in number_vectors.iter() {
        totalsum += sum;
    }

    println!("{}", totalsum)
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename).unwrap().lines().map(String::from).collect()
}

fn get_digits_in_string(s: &str) -> Vec<u32> {
    let mut digits:Vec<u32> = Vec::new();
    let chars_vec:Vec<char> = s.chars().collect();
    
    for (i, char) in chars_vec.iter().enumerate() {
        match char.to_digit(10) {
            Some(digit) => {
                digits.push(digit); 
                continue
            },
            None => (),
        }
        match find_word(i, &chars_vec) {
            Some(digit) => digits.push(digit),
            None => continue,
        }
    }

    digits
}

fn find_word(i: usize, chars: &Vec<char>) -> Option<u32> {
    let first_letters:Vec<char> = vec!['z','o','t','f','s','e','n'];
    let word_digit:HashMap<&str, u32> = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    if first_letters.contains(&chars[i]) {
        let mut test_word:String = chars[i].to_string();
        for j in 1..5 {
            if i + j >= chars.len() {
                continue;
            }
            test_word += &chars[i + j].to_string();
            if word_digit.contains_key(&test_word.as_str()) {
                return Some(word_digit[&test_word.as_str()])
            }
        }
        None
    }
    else {
        None
    }
}

fn add_first_and_last_digit_from_vector(v: &Vec<u32>) -> u32 {
    let mut sum:u32 = 0;
    match v.first() {
        Some(d) => sum += d * 10,
        None => panic!("wtf"),
    }
    match v.last() {
        Some(d) => sum += d,
        None => panic!("wtf"),
    }

    sum
}