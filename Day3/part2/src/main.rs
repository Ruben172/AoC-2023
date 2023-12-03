use std::fs::read_to_string;

fn main() {
    let grid = read_lines("input");
    let mut has_asterisk:Vec<(u32, u32, u32)> = Vec::new();
    let mut ratio_sum = 0;
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
                    let asterisk = range_has_asterisk(row as i32, col as i32, number.to_string().len() as i32, &grid);
                    if asterisk.is_some() {
                        has_asterisk.push((asterisk.unwrap().0, asterisk.unwrap().1, number))
                    }
                },
                None => continue,
            }
        }
    }
    
    for (i, asterisk) in has_asterisk.iter().enumerate() {
        let mut adjacencies = 0;
        let mut other_number = 0;
        for other_asterisk in has_asterisk[i..].iter() {
            if asterisk.0 == other_asterisk.0 && asterisk.1 == other_asterisk.1 {
                adjacencies += 1;
                other_number = other_asterisk.2;
            }
        }

        if adjacencies == 2 {
            let gear_ratio = asterisk.2 * other_number;
            ratio_sum += gear_ratio;
        }
    }

    dbg!(ratio_sum);
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

fn range_has_asterisk(row: i32, col: i32, len: i32, grid: &Vec<String>) -> Option<(u32,u32)> {
    let grid_rows = grid.len() as i32;
    let grid_cols = grid[0].len() as i32;
    for c in -1..=len {
        for r in -1..=1 {
            if row+r > 0 && row+r < grid_rows {
                if col+c > 0 && col+c < grid_cols {
                    match grid.get((row+r) as usize).unwrap().chars().nth((col + c) as usize).unwrap() == '*' {
                        true => return Some(((row+r) as u32, (col+c) as u32)),
                        false => (),
                    }
                }
            }
        }
    }
    None
}