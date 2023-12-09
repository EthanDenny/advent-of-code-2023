pub fn answers(input: Vec<String>) -> (i32, i32) {
    let mut ans1 = 0;
    let mut ans2 = 0;

    for line in input {
        // Parsing

        let nums = line
            .split(" ")
            .map(|e| e.parse().unwrap())
            .collect::<Vec<i32>>();

        let mut layers = vec![nums];

        let mut all_zeroes = false;

        while !all_zeroes {
            let mut old_layer = layers[layers.len() - 1].iter().peekable();
            let mut new_layer = Vec::new();
            all_zeroes = true;

            while let Some(m) = old_layer.next() {
                if let Some(&n) = old_layer.peek() {
                    if m != n {
                        all_zeroes = false;
                    }
                    new_layer.push(n - m);
                }
            }

            layers.push(new_layer);
        }

        // Answer 1

        let mut last_edge = 0;

        for layer in layers.iter_mut().rev() {
            last_edge += layer[layer.len() - 1];
        }

        ans1 += last_edge;

        // Answer 2

        let mut last_edge = 0;

        for layer in layers.iter().rev() {
            last_edge = layer[0] - last_edge;
        }

        ans2 += last_edge;
    }

    // Return

    (ans1, ans2)
}
