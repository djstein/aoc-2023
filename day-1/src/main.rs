use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::u32;

const ONE_NUM: &str = "1";
const ONE_STR: &str = "one";
const TWO_NUM: &str = "2";
const TWO_STR: &str = "two";
const THREE_NUM: &str = "3";
const THREE_STR: &str = "three";
const FOUR_NUM: &str = "4";
const FOUR_STR: &str = "four";
const FIVE_NUM: &str = "5";
const FIVE_STR: &str = "five";
const SIX_NUM: &str = "6";
const SIX_STR: &str = "six";
const SEVEN_NUM: &str = "7";
const SEVEN_STR: &str = "seven";
const EIGHT_NUM: &str = "8";
const EIGHT_STR: &str = "eight";
const NINE_NUM: &str = "9";
const NINE_STR: &str = "nine";

fn main() {
    let mut digits: Vec<u32> = Vec::new();
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                get_digits_for_line(line, &mut digits)
            }
        }
    }
    let sum_of_all_digits = sum_digits(&mut digits);
    println!("Sum of all digits: {}", sum_of_all_digits);
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

fn convert_str_rep_to_num_str_rep(num_as_str: &str) -> String {
    match num_as_str {
        ONE_STR => ONE_NUM.to_string(),
        TWO_STR => TWO_NUM.to_string(),
        THREE_STR => THREE_NUM.to_string(),
        FOUR_STR => FOUR_NUM.to_string(),
        FIVE_STR => FIVE_NUM.to_string(),
        SIX_STR => SIX_NUM.to_string(),
        SEVEN_STR => SEVEN_NUM.to_string(),
        EIGHT_STR => EIGHT_NUM.to_string(),
        NINE_STR => NINE_NUM.to_string(),
        _ => num_as_str.to_string(),
    }
}

fn add_digit_to_line_digits(
    current_index: usize,
    search_value: String,
    lowest_index: &mut usize,
    first_value: &mut String,
    highest_index: &mut usize,
    last_value: &mut String,
) {
    println!(
        "Found search value: {} at current_index: {}",
        search_value, current_index
    );
    if current_index <= *lowest_index {
        *lowest_index = current_index;
        *first_value = search_value.clone();
    }
    if current_index >= *highest_index {
        *highest_index = current_index;
        *last_value = search_value.clone();
    }
}

fn get_line_digits(line_digits: &mut Vec<String>, line: String) {
    let mut lowest_index: usize = 99;
    let mut first_value: String = String::new();
    let mut highest_index = 0;
    let mut last_value: String = String::new();

    for search_value in vec![
        ONE_STR, TWO_STR, THREE_STR, FOUR_STR, FIVE_STR, SIX_STR, SEVEN_STR, EIGHT_STR, NINE_STR,
        ONE_NUM, TWO_NUM, THREE_NUM, FOUR_NUM, FIVE_NUM, SIX_NUM, SEVEN_NUM, EIGHT_NUM, NINE_NUM,
    ] {
        // println!(
        //     "Search value: {} Lowest index: {} First value: {} Highest index: {} Last value: {}",
        //     search_value, lowest_index, first_value, highest_index, last_value
        // );
        line.find(search_value).and_then(|current_index: usize| {
            Some(add_digit_to_line_digits(
                current_index,
                String::from(search_value),
                &mut lowest_index,
                &mut first_value,
                &mut highest_index,
                &mut last_value,
            ))
        });

        // had to do a rfind because find only got the first value of the search
        // this did not account for if there was TWO or more of the same number in the line
        line.rfind(search_value).and_then(|current_index: usize| {
            Some(add_digit_to_line_digits(
                current_index,
                String::from(search_value),
                &mut lowest_index,
                &mut first_value,
                &mut highest_index,
                &mut last_value,
            ))
        });
    }

    // println!("Lowest index: {} Highest: {}", lowest_index, highest_index);
    // println!("First Value: {} Last Value: {}", first_value, last_value);
    line_digits.push(convert_str_rep_to_num_str_rep(&first_value));
    line_digits.push(convert_str_rep_to_num_str_rep(&last_value));
}

fn get_digits_for_line(line: String, digits: &mut Vec<u32>) {
    // println!("Line: {:?}", line);
    let mut line_digits: Vec<String> = Vec::new();
    get_line_digits(&mut line_digits, line.clone());

    println!("Line digits: {:?} for Line: {:?}", line_digits, line);
    let new_number: u32 = format!("{}{}", line_digits[0], line_digits[1])
        .parse::<u32>()
        .unwrap();
    digits.push(new_number);
}

fn sum_digits(digits: &mut Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    for current_line_sum in digits {
        sum += *current_line_sum;
    }
    sum
}

// for (pos, c) in line.chars().enumerate() {
//     if c.is_digit(10) {
//         let current_digit: u32 = c.to_digit(10).unwrap();
//         if line_digits.len() == 0 {
//             line_digits.push(current_digit);
//         } else if line_digits.len() == 2 {
//             line_digits[1] = current_digit;
//         } else {
//             line_digits.push(current_digit);
//         }
//     }
//     if pos == line.len() - 1 {
//         if line_digits.len() == 1 {
//             line_digits.push(line_digits[0]);
//         }
//     }
// }
