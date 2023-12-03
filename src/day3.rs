pub fn answers(input: Vec<String>) -> (i32, i32) {
    let mut ans1 = 0;
    let mut ans2 = 0;

    let it = input.iter().enumerate();

    let mut gear_nums: Vec<(i32, (usize, usize))> = Vec::new();
    let mut gear_pos: (usize, usize) = (0, 0);

    for (y, line) in it {
        let mut num = String::new();
        let mut found_symbol = false;
        let mut found_gear = false;

        let mut long_line = String::from(line);
        long_line.push('.');
        let ch = long_line.chars().enumerate();

        for (x, c) in ch {
            match c {
                '0'..='9' => {
                    num.push(c);
                    for yd in -1..=1 {
                        let new_y = (y as i32 + yd) as usize;
                        if (y > 0 || yd > -1) && (new_y < input.len()) {
                            let line = input.iter().nth(new_y).unwrap();
                            for xd in -1..=1 {
                                let new_x = (x as i32 + xd) as usize;
                                if (x > 0 || xd > -1) && (new_x < input.len()) {
                                    let c = line.chars().nth(new_x).unwrap();
                                    match c {
                                        '0'..='9' | '.' => {}
                                        '*' => {
                                            found_symbol = true;
                                            found_gear = true;
                                            gear_pos = (new_x, new_y);
                                        }
                                        _ => found_symbol = true
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    if found_symbol && num.len() > 0 {
                        let n = num.parse::<i32>().unwrap();
                        ans1 += n;
                        if found_gear {
                            gear_nums.push((n, gear_pos));
                            found_gear = false;
                        }
                    }
                    num = String::new();
                    found_symbol = false;
                }
            }
        }
    }

    for (i, (a, gear_a)) in gear_nums.iter().enumerate() {
        for (b, gear_b) in gear_nums.iter().skip(i+1) {
            if gear_a == gear_b {
                ans2 += a * b;
            }
        }
    }

    (ans1, ans2)
}
