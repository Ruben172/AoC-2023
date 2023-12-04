use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let lines = read_lines("input");
    let mut duplicates: HashMap<u32, u32> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        let card_count:u32;
        match duplicates.get(&(i as u32)) {
            Some(v) => card_count = v.to_owned(),
            None => {
                duplicates.insert(i as u32, 1);
                card_count = 1;
            }
        }

        let (winning, card) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let winning_numbers = add_numbers(&winning);
        let card_numbers = add_numbers(&card);
        let mut amount_winning = 0;

        for number in &card_numbers {
            if winning_numbers.contains(number) {
                amount_winning += 1;
            }
        }

        for j in 1..=amount_winning {
            let key = &(i as u32 + j);
            if key > &(lines.len() as u32) {
                break;
            }

            _ = match duplicates.get(key) {
                None => duplicates.insert(key.to_owned(), 1 + card_count),
                Some(v) => duplicates.insert(key.to_owned(), v + card_count),
            }
        }
    }

    let mut total_cards = 0;
    for k in duplicates.keys().into_iter() {
        total_cards += duplicates.get(k).unwrap();
    }
    dbg!(total_cards);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename).unwrap().lines().map(String::from).collect()
}

fn add_numbers(s: &str) -> Vec<u32> {
    let mut v = Vec::new();
    for number in s.split(" ") {
        match number.parse::<u32>() {
            Ok(number) => v.push(number),
            Err(_) => continue,
        }
    }

    v
}