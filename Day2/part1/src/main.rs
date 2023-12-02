use std::fs::read_to_string;

fn main() {
    let lines = read_lines("input");
    let mut gameSum = 0;
    for (i, line) in lines.iter().enumerate() {
        let sets = line.split_once(": ").unwrap().1;
        let mut possible = true;
        for set in sets.split("; ") {
            for cubes in set.split(", ") {
                let (amountstr, color) = cubes.split_once(" ").unwrap();
                let amount = amountstr.parse::<u32>().unwrap();
                match color {
                    "red" => if amount > 12 {
                        possible = false;
                    }
                    "green" => if amount > 13 {
                        possible = false;
                    }
                    "blue" => if amount > 14 {
                        possible = false;
                    }
                    &_ => panic!("!??!!")
                }
            }
        }
        if possible {
            gameSum += i + 1;
        }
    }

    dbg!(gameSum);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename).unwrap().lines().map(String::from).collect()
}
