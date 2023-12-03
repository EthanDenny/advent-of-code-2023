mod day1;
mod day2;

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
    let date = chrono::Utc::now().day() as i32;

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

    let (ans1, ans2) = match day {
        1 => day1::answers(&input),
        2 => day2::answers(&input),
        _ => unimplemented!(),
    };

    println!("Answer 1: {ans1}");
    println!("Answer 2: {ans2}");
}
