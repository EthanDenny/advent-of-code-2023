struct GardeningMap {
    dest_start: i64,
    source_start: i64,
    range: i64
}

impl GardeningMap {
    fn map(&self, val: i64) -> Option<i64> {
        if self.source_start <= val && val < self.source_start + self.range {
            Some(self.dest_start + (val - self.source_start))
        } else {
            None
        }
    }
}

pub fn answers(input: Vec<String>) -> (i64, i64) {
    // Parsing

    let mut lines = input.iter();
    

    let seeds: Vec<i64> = lines.next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|e| e.parse::<i64>().unwrap())
        .collect();

    lines.next();

    let mut map_functions: Vec<Vec<GardeningMap>> = Vec::new();
    let mut curr_mapf: Vec<GardeningMap> = Vec::new();

    while let Some(line) = lines.next() {
        if line.len() == 0 {
            map_functions.push(curr_mapf);
            curr_mapf = Vec::new();
            continue;
        }

        match line.chars().nth(0).unwrap() {
            '0'..='9' => {
                let parts: Vec<i64> = line.split(' ')
                    .map(|e| e.parse::<i64>().unwrap())
                    .collect();

                curr_mapf.push(GardeningMap {
                    dest_start: parts[0],
                    source_start: parts[1],
                    range: parts[2]
                })
            },
            _ => ()
        }
    }

    map_functions.push(curr_mapf);

    // Answer 1

    let mut ans1 = i64::MAX;

    for seed in seeds.iter() {
        let mut current_target_val = *seed;

        for mapf in map_functions.iter() {
            for gmap in mapf {
                if let Some(val) = gmap.map(current_target_val) {
                    current_target_val = val;
                    break;
                }
            }
        }

        if ans1 > current_target_val {
            ans1 = current_target_val;
        }
    }

    // Answer 2
    // Brute forced it because why not

    let mut ans2: i64 = i64::MAX;

    let mut seeds = seeds.iter();

    while let Some(&source_start) = seeds.next() {
        let range = *seeds.next().unwrap();

        for seed in source_start..source_start+range {
            let mut current_target_val = seed;
    
            for mapf in map_functions.iter() {
                for gmap in mapf {
                    if let Some(val) = gmap.map(current_target_val) {
                        current_target_val = val;
                        break;
                    }
                }
            }
    
            if ans2 > current_target_val {
                ans2 = current_target_val;
            }
        }
    }

    (ans1, ans2)
}
