mod day1;
mod day2;
mod day3;
mod day4;
mod day6;
mod day7;

use chrono::{Datelike, FixedOffset, Utc};
use std::env;
use std::fs;

// Adapted from the documentation
fn read_lines(day: i32) -> Vec<String> {
    let path = format!("input/day{day}.txt");
    let message = format!("Couldn't open input for Day {day}");

    fs::read_to_string(&path).expect(&message).lines().map(String::from).collect()
}

fn get_answers(input: Vec<String>, day: i32) -> (i32, i32) {
    match day {
        1 => day1::answers(input),
        2 => day2::answers(input),
        3 => day3::answers(input),
        4 => day4::answers(input),
        6 => day6::answers(input),
        7 => day7::answers(input),
        _ => unimplemented!()
    }
}

fn main() {
    let eastern = FixedOffset::west_opt(5 * 3600).unwrap();
    let date = Utc::now().with_timezone(&eastern).day() as i32;

    let day: i32 = {
        if let Some(arg) = env::args().nth(1) {
            let arg = arg.parse().unwrap();
            if arg > date {
                panic!("It isn't Day {arg} yet!");
            } else {
                arg
            }
        } else {
            date
        }
    };

    let input = read_lines(day);
    let (ans1, ans2) = get_answers(input, day);

    println!("Answer 1: {ans1}");
    println!("Answer 2: {ans2}");
}
