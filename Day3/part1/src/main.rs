use std::fs::read_to_string;

fn main() {
    let grid = read_lines("input");
    let mut partsSum = 0;
    for (row, l) in grid.iter().enumerate() {
        for (col, c) in l.chars().enumerate() {
            if col != 0 {
                match grid.get(row).unwrap().chars().nth(col-1).unwrap().to_digit(10) {
                    Some(_) => continue,
                    None => (),
                }
            }
            match c.to_digit(10) {
                Some(_) => {
                    let number = parse_int(&l[col..]);
                    if range_is_part(row as i32, col as i32, number.to_string().len() as i32, &grid) {
                        partsSum += number;
                    }
                },
                None => continue,
            }
        }
    }
    dbg!(partsSum);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename).unwrap().lines().map(String::from).collect()
}

fn parse_int(s: &str) -> u32 {
    let mut number = String::new();
    for c in s.chars() {
        match c.to_digit(10) {
            Some(_) => number += &c.to_string(),
            None => break,
        }
    }

    number.parse::<u32>().unwrap()
}

fn range_is_part(row: i32, col: i32, len: i32, grid: &Vec<String>) -> bool {
    let gridRows = grid.len() as i32;
    let gridCols = grid[0].len() as i32;
    for c in -1..=len {
        for r in -1..=1 {
            if row+r > 0 && row+r < gridRows {
                if col+c > 0 && col+c < gridCols {
                    match is_symbol(grid.get((row+r) as usize).unwrap().chars().nth((col + c) as usize).unwrap()) {
                        true => return true,
                        false => (),
                    }
                }
            }
        }
    }
    false
}

fn is_symbol(c: char) -> bool {
    if c.to_digit(10).is_some() {
        false
    }
    else if c == '.' {
        false
    }
    else {
        true
    }
}