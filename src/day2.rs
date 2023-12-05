const MAX_RED: i32 = 12;
const MAX_BLUE: i32 = 13;
const MAX_GREEN: i32 = 14;

pub fn answers(input: Vec<String>) -> (i32, i32) {
    let mut ans1 = 0;
    let mut ans2 = 0;

    for game in input {
        let colon_index: usize = game.find(":").unwrap();

        let mut id = (&game[5..colon_index]).parse().unwrap();
        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;

        for subset in (&game[colon_index+2..]).split("; ") {
            for cube in subset.split(", ") {
                let space_index = cube.find(" ").unwrap();

                let count: i32 = (&cube[..space_index]).parse().unwrap();
                let colour: &str = &cube[space_index+1..];

                match colour {
                    "red" => if count > MAX_RED { id = 0; }
                    "blue" => if count > MAX_BLUE { id = 0; }
                    "green" => if count > MAX_GREEN { id = 0; }
                    _ => {}
                }

                match colour {
                    "red" => if count > min_red { min_red = count; }
                    "blue" => if count > min_blue { min_blue = count; }
                    "green" => if count > min_green { min_green = count; }
                    _ => {}
                }
            }
        }

        ans1 += id;
        ans2 += min_red * min_blue * min_green;
    }

    (ans1, ans2)
}
