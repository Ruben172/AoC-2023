use std::fs::read_to_string;

fn main() {
    let lines = read_lines("input");
    let mut number_vectors:Vec<u32> = Vec::new();
    let mut totalsum:u32 = 0;
    for line in lines.iter() {
        number_vectors.push(add_first_and_last_digit_from_string(line));
    }
    for sum in number_vectors.iter() {
        totalsum += sum;
    }

    println!("{}", totalsum)
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename).unwrap().lines().map(String::from).collect()
}

fn add_first_and_last_digit_from_string(s: &str) -> u32 {
    let mut digits:Vec<u32> = Vec::new();
    
    for char in s.chars() {
        match char.to_digit(10) {
            Some(digit) => digits.push(digit),
            None => continue,
        }
    }

    let mut sum:u32 = 0;
    match digits.first() {
        Some(d) => sum += d * 10,
        None => panic!("wtf"),
    }
    match digits.last() {
        Some(d) => sum += d,
        None => panic!("wtf"),
    }

    sum
}