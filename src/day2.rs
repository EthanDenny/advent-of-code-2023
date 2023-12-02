pub fn answer1(input: &Vec<String>) -> i32 {
    input.iter()
        .map(|game: &String| check_valid(game, 12, 13, 14))
        .sum()
}

pub fn answer2(input: &Vec<String>) -> i32 {
    input.iter()
        .map(|game: &String| get_power(game))
        .sum()
}

fn check_valid(game: &String, red: i32, green: i32, blue: i32) -> i32 {
    let colon_index: usize = game.find(":").unwrap();

    let id: i32 = (&game[5..colon_index]).parse().unwrap();
    let subsets: Vec<&str> = (&game[colon_index+2..]).split("; ").collect();

    for subset in subsets {
        let cubes: Vec<&str> = subset.split(", ").collect();

        for cube in cubes {
            let space_index = cube.find(" ").unwrap();

            let count: i32 = (&cube[..space_index]).parse().unwrap();
            let colour: &str = &cube[space_index+1..];

            match colour {
                "red" => if count > red { return 0; }
                "blue" => if count > blue { return 0; }
                "green" => if count > green { return 0; }
                _ => {}
            }
        }
    }

    id
}

fn get_power(game: &String) -> i32 {
    let mut min_red = 0;
    let mut min_blue = 0;
    let mut min_green = 0;

    let colon_index: usize = game.find(":").unwrap();

    let subsets: Vec<&str> = (&game[colon_index+2..]).split("; ").collect();

    for subset in subsets {
        let cubes: Vec<&str> = subset.split(", ").collect();

        for cube in cubes {
            let space_index = cube.find(" ").unwrap();

            let count: i32 = (&cube[..space_index]).parse().unwrap();
            let colour: &str = &cube[space_index+1..];

            match colour {
                "red" => if count > min_red { min_red = count; }
                "blue" => if count > min_blue { min_blue = count; }
                "green" => if count > min_green { min_green = count; }
                _ => {}
            }
        }
    }

    min_red * min_blue * min_green
}
