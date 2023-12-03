use core::panic;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::u32;

fn main() {
    // part_1();
    part_2();
}

fn _part_1() {
    let mut possible_game_ids: Vec<u32> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            println!("Line: {:?}", line);
            if let Ok(line) = line {
                _valid_game(line, &mut possible_game_ids)
            }
        }
    }
    let sum_all_ids = sum_digits(&mut possible_game_ids);
    println!("sum_all_ids: {}", sum_all_ids);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn _get_game_id(game_with_id_str: String) -> u32 {
    let game_id_str: Vec<&str> = game_with_id_str.split("Game ").collect();
    let game_id_num: u32 = String::from(game_id_str[1]).parse::<u32>().unwrap();
    game_id_num
}

fn _valid_game(line: String, possible_game_ids: &mut Vec<u32>) {
    let game_and_values: Vec<&str> = line.split(":").collect();
    let game_with_id_str: String = String::from(game_and_values[0]);
    let game_id_num: u32 = _get_game_id(game_with_id_str);

    let red_cubes_max: u32 = 12;
    let green_cubes_max: u32 = 13;
    let blue_cubes_max: u32 = 14;

    let mut valid_game: bool = true;

    let binding: String = game_and_values[1].replace(";", ",");
    let values: Vec<&str> = binding.split(",").collect();

    for value in values {
        let trimmed_value = value.trim();
        let value_str: Vec<&str> = trimmed_value.split(" ").collect();
        let color: &str = value_str[1];
        println!("color: {:?}", color);
        let value_num: u32 = String::from(value_str[0]).parse::<u32>().unwrap();
        println!("value_num: {:?}", value_num);
        valid_game = match color {
            "red" => !(value_num > red_cubes_max),
            "green" => !(value_num > green_cubes_max),
            "blue" => !(value_num > blue_cubes_max),
            _ => panic!("Invalid color"),
        };
        if valid_game == false {
            println!("Invalid game: {}", game_id_num);
            break;
        }
    }
    if valid_game == true {
        println!("Valid game: {}", game_id_num);
        possible_game_ids.push(game_id_num);
    }
}

fn sum_digits(digits: &mut Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    for digit in digits {
        sum += digit.to_owned();
    }
    sum
}

fn part_2() {
    let mut game_powers: Vec<u32> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            println!("Line: {:?}", line);
            if let Ok(line) = line {
                get_powers(line, &mut game_powers)
            }
        }
    }
    let sum_game_powers = sum_digits(&mut game_powers);
    println!("sum_game_powers: {}", sum_game_powers);
}

fn get_powers(line: String, game_powers: &mut Vec<u32>) {
    let mut total_red_for_game: u32 = 0;
    let mut total_green_for_game: u32 = 0;
    let mut total_blue_for_game: u32 = 0;

    let game_and_values: Vec<&str> = line.split(":").collect();
    let binding: String = game_and_values[1].replace(";", ",");
    let values: Vec<&str> = binding.split(",").collect();
    for value in values {
        let trimmed_value = value.trim();
        let value_str: Vec<&str> = trimmed_value.split(" ").collect();
        let color: &str = value_str[1];
        let value_num: u32 = String::from(value_str[0]).parse::<u32>().unwrap();
        match color {
            "red" => {
                if value_num > total_red_for_game {
                    total_red_for_game = value_num;
                }
            }
            "green" => {
                if value_num > total_green_for_game {
                    total_green_for_game = value_num;
                }
            }
            "blue" => {
                if value_num > total_blue_for_game {
                    total_blue_for_game = value_num;
                }
            }
            _ => panic!("Invalid color"),
        };
    }
    game_powers.push(total_blue_for_game * total_green_for_game * total_red_for_game);
}
