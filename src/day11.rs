use std::collections::HashSet;

pub fn answers(input: Vec<String>) -> (i32, u64) {
    // Parsing

    let mut galaxies = HashSet::new();
    let mut expanded_x = HashSet::new();
    let mut expanded_y = HashSet::new();

    for (y, line) in input.iter().enumerate() {
        expanded_y.insert(y);
        for (x, ch) in line.chars().enumerate() {
            if y == 0 {
                expanded_x.insert(x);
            }
            if ch == '#' {
                galaxies.insert((x, y));
                expanded_x.remove(&x);
                expanded_y.remove(&y);
            }
        }
    }

    // Answer 1 / 2

    let mut ans1 = 0;
    let mut ans2: u64 = 0;

    for (i, galaxy_a) in galaxies.iter().enumerate() {
        for galaxy_b in galaxies.iter().skip(i+1) {
            let (x_max, x_min) = max_min(galaxy_a.0, galaxy_b.0);
            let (y_max, y_min) = max_min(galaxy_a.1, galaxy_b.1);

            for x in x_min..x_max {
                ans1 += if expanded_x.contains(&x) { 2 } else { 1 };
                ans2 += if expanded_x.contains(&x) { 1000000 } else { 1 };
            }

            for y in y_min..y_max {
                ans1 += if expanded_y.contains(&y) { 2 } else { 1 };
                ans2 += if expanded_y.contains(&y) { 1000000 } else { 1 };
            }
        }
    }

    // Return

    (ans1, ans2)
}

fn max_min(a: usize, b: usize) -> (usize, usize) {
    if a >= b {
        (a, b)
    } else {
        (b, a)
    }
}
