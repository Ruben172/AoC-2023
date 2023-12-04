use std::fs::read_to_string;

fn main() {
    let lines = read_lines("input");
    let mut total_points = 0;

    for line in lines {
        let winning;
        let card;
        let mut winning_numbers = Vec::new();
        let mut card_numbers = Vec::new();
        (winning, card) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        for number in winning.split(" ") {
            match number.parse::<u32>() {
                Ok(number) => winning_numbers.push(number),
                Err(_) => continue,
            }
        }
        for number in card.split(" ") {
            match number.parse::<u32>() {
                Ok(number) => card_numbers.push(number),
                Err(_) => continue,
            }
        }
        
        let mut cardpoints = 0.5;
        for number in card_numbers {
            if winning_numbers.contains(&number) {
                cardpoints *= 2.0;
            }
        }

        total_points += cardpoints as u32;
    }
    dbg!(total_points);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename).unwrap().lines().map(String::from).collect()
}
