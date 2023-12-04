pub fn answers(input: Vec<String>) -> (i32, i32) {
    let matches: Vec<i32> = input.iter()
        .map(|e| count_matches(e))
        .collect();

    let mut instances: Vec<i32> = vec![1; input.len()];

    for (idx, &m) in matches.iter().enumerate() {
        let count = instances[idx];

        if count == 0 {
            break;
        }

        for idx_offset in 1..=m {
            instances[idx + idx_offset as usize] += count;
        }
    }

    let ans1 = matches.iter()
        .map(|&e| {
            if e <= 1 {
                e
            } else {
                i32::pow(2, e as u32 - 1)
            }
        })
        .sum();

    let ans2 = instances.iter().sum();

    (ans1, ans2)
}

fn count_matches(line: &String) -> i32 {
    let colon_index = line.find(":").unwrap();
    let bar_index = line.find(" | ").unwrap();

    let my_nums: Vec<_> = (&line[colon_index+2..bar_index])
        .split(" ")
        .filter(|&e| e != "")
        .collect();
    let win_nums: Vec<_> = (&line[bar_index+3..])
        .split(" ")
        .filter(|&e| e != "")
        .collect();

    my_nums.iter()
        .map(|num| {
            match win_nums.iter().find(|&e| e == num) {
                Some(_) => 1,
                None => 0
            }
        })
        .sum()
}
