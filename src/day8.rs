use std::collections::HashMap;
use std::collections::HashSet;
use gcd::Gcd;

pub fn answers(input: Vec<String>) -> (i32, u64) {
    // Parsing

    let mut lines = input.iter();
    let instructions = lines.next().unwrap();
    lines.next();

    let mut tree = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let parent = parts[0];
        let left = &(parts[2])[1..parts[2].len()-1];
        let right = &(parts[3])[..parts[3].len()-1];

        tree.insert(parent, (left, right));
    }

    // Answer 1

    let mut ans1 = 0;

    let mut curr = "AAA";

    while curr != "ZZZ" {
        for direction in instructions.chars() {
            match direction {
                'L' => curr = tree.get(curr).unwrap().0,
                'R' => curr = tree.get(curr).unwrap().1,
                _ => {}
            }
        }
        ans1 += instructions.len() as i32;
    }

    // Answer 2

    let mut start_nodes = HashSet::new();
    let mut step_counts = Vec::new();

    for node in tree.keys() {
        if node.chars().nth(2) == Some('A') {
            start_nodes.insert(*node);
        }
    }

    for start in start_nodes.iter() {
        let mut curr = *start;
        let mut steps = 0;

        while curr.chars().nth(2) != Some('Z') {
            for direction in instructions.chars() {
                match direction {
                    'L' => curr = tree.get(curr).unwrap().0,
                    'R' => curr = tree.get(curr).unwrap().1,
                    _ => {}
                }
            }
            steps += instructions.len();
        }

        step_counts.push(steps);
    }

    let mut ans2: u64 = 1;
    let lcm = |a: u64, b: usize| a * b as u64 / a.gcd(b as u64);

    for count in step_counts {
        ans2 = lcm(ans2, count);
    }

    // Return

    (ans1, ans2)
}
