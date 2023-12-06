pub fn answers(input: Vec<String>) -> (i32, i32) {
    // Parsing

    let parse_small = |s: &String| {
        s
        .split(' ')
        .filter(|&e| e != "")
        .skip(1)
        .map(|e| e.parse().unwrap())
        .collect::<Vec<i64>>()
    };

    let parse_big = |s: &String| {
        s
        .replace(" ", "")
        .split(':')
        .nth(1)
        .unwrap()
        .parse::<i64>()
        .unwrap()
    };

    let record_breakers = |t, d| {
        let mut count = 0;

        for held in 0..=t {
            if (t - held) * held > d {
                count += 1;
            }
        }

        count
    };

    // Answer 1

    let times = parse_small(&input[0]);
    let distances = parse_small(&input[1]);

    let ans1 = times.into_iter()
        .zip(distances)
        .map(|(t, d)| record_breakers(t, d))
        .product();

    // Answer 2

    let big_time = parse_big(&input[0]);
    let big_distance = parse_big(&input[1]);

    let ans2 = record_breakers(big_time, big_distance);

    // Return

    (ans1, ans2)
}
