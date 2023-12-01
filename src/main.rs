mod day1;

use chrono::Datelike;
use std::env;
use std::fs;

// Adapted from the documentation
fn read_lines(day: i32) -> Vec<String> {
    let path = format!("input/day{day}.txt");
    let message = format!("Couldn't open input for day {day}");

    fs::read_to_string(&path).expect(&message).lines().map(String::from).collect()
}

fn main() {
    let day: i32 = {
        if let Some(arg) = env::args().nth(1) {
            arg.parse().unwrap()
        } else {
            chrono::Utc::now().day() as i32
        }
    };

    let input = read_lines(day);

    let answer1 = match day {
        1 => day1::answer1(&input),
        _ => String::from("Day not found")
    };

    let answer2 = match day {
        1 => day1::answer2(&input),
        _ => String::from("Day not found")
    };

    println!("Answer 1: {answer1}");
    println!("Answer 2: {answer2}");
}
