use std::collections::HashMap;
use std::cmp::Ordering;

pub fn answers(input: Vec<String>) -> (i32, i32) {
    let mut hands = input
        .iter()
        .map(|e| {
            let parts: Vec<&str> = e.split(' ').collect();
            let hand = parts[0];
            let bid = parts[1].parse::<i32>().unwrap();
            (hand, bid)
        })
        .collect::<Vec<(&str, i32)>>();
    
    let ans1 = get_hand_sum(&mut hands, false);
    let ans2 = get_hand_sum(&mut hands, true);

    (ans1, ans2)
}

fn get_hand_sum(hands: &mut Vec<(&str, i32)>, jokers_low: bool) -> i32 {
    hands.sort_by(|a, b| rank_hands(b.0, a.0, jokers_low));
    
    hands.iter()
        .enumerate()
        .map(|e| (e.0 as i32 + 1) * e.1.1)
        .sum()
}

fn rank_hands(hand_a: &str, hand_b: &str, jokers_low: bool) -> Ordering {
    let type_a = get_hand_type(hand_a, jokers_low);
    let type_b = get_hand_type(hand_b, jokers_low);

    if type_a == type_b {
        for (card_a, card_b) in hand_a.chars().zip(hand_b.chars()) {
            let value_a = get_card_value(card_a, jokers_low);
            let value_b = get_card_value(card_b, jokers_low);
            let ord = value_b.partial_cmp(&value_a).unwrap();
            match ord {
                Ordering::Equal => {},
                _ => return ord
            }
        }
    }

    type_a.partial_cmp(&type_b).unwrap()
}

fn get_card_value(card: char, jokers_low: bool) -> i32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => if jokers_low { 1 } else { 11 },
        'T' => 10,
        _ => card.to_digit(10).unwrap() as i32
    }
}

fn get_hand_type(hand: &str, jokers_low: bool) -> i32 {
    let mut cards: HashMap<char, i32> = HashMap::new();

    for c in hand.chars() {
        *cards.entry(c).or_insert(0) += 1;
    }

    if jokers_low {
        let max_pair = *cards.iter().max_by_key(|e| if e.0 == &'J' { &0 } else { e.1 }).unwrap().0;
        let j = cards.get(&'J');
        if let Some(&j_count) = j {
            if cards.len() == 1 {
                return 0;
            } else{
                cards.entry(max_pair).and_modify(|e| { *e += j_count });
                cards.remove(&'J');
            }
        }
    }

    match cards.len() {
        1 => 0, // AAAAA
        2 => {
            let mut rank = 1; // Assume AAAAB

            for (_, count) in cards {
                if count == 2 {
                    rank = 2; // AAABB
                    break;
                }
            }

            rank
        }
        3 => {
            let mut rank = 3; // Assume AAABC

            for (_, count) in cards {
                if count == 2 {
                    rank = 4; // AABBCC
                    break;
                }
            }

            rank
        }
        4 => 5, // AABCD
        5 => 6, // ABCDE
        _ => i32::MAX
    }
}
