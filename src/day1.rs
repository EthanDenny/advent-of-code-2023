pub fn answer1(input: &Vec<String>) -> String {
    let matches = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    get_calibration(input, &matches, &to_char)
}

pub fn answer2(input: &Vec<String>) -> String {
    let matches = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9",
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ];
    get_calibration(input, &matches, &to_digit)
}

fn get_calibration(input: &Vec<String>, matches: &[&str], conversion: &dyn Fn(&str) -> char) -> String {
    let mut sum: u32 = 0;

    for line in input {
        let mut digit1 = '0';
        let mut digit2 = '0';

        let mut first_index: usize = usize::MAX;
        let mut last_index: usize = 0;

        for m in matches {
            if let Some(this_first_index) = line.find(m) {
                if this_first_index <= first_index {
                    first_index = this_first_index;
                    digit1 = conversion(m);
                }
            }

            if let Some(this_last_index) = line.rfind(m) {
                if this_last_index >= last_index {
                    last_index = this_last_index;
                    digit2 = conversion(m);
                }
            }
        }

        sum += digit1.to_digit(10).unwrap() * 10;
        sum += digit2.to_digit(10).unwrap();
    }

    sum.to_string()
}

fn to_char(s: &str) -> char{
    s.chars().nth(0).unwrap()
}

fn to_digit(s: &str) -> char {
    match s {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => to_char(s)
    }
}
