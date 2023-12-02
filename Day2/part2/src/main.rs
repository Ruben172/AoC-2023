use std::fs::read_to_string;

fn main() {
    let lines = read_lines("input");
    let mut powerSum = 0;
    for (i, line) in lines.iter().enumerate() {
        let sets = line.split_once(": ").unwrap().1;
        let mut maxRed = 0;
        let mut maxGreen = 0;
        let mut maxBlue = 0;
        for set in sets.split("; ") {
            for cubes in set.split(", ") {
                let (amountstr, color) = cubes.split_once(" ").unwrap();
                let amount = amountstr.parse::<u32>().unwrap();
                match color {
                    "red" => if amount > maxRed {
                        maxRed = amount;
                    }
                    "green" => if amount > maxGreen {
                        maxGreen = amount;
                    }
                    "blue" => if amount > maxBlue {
                        maxBlue = amount;
                    }
                    &_ => panic!("!??!!")
                }
            }
        }
        let power = maxRed * maxGreen * maxBlue;
        powerSum += power;
    }

    dbg!(powerSum);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename).unwrap().lines().map(String::from).collect()
}
