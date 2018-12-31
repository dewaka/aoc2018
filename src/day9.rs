use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::iter::Iterator;

#[inline]
fn update_row(
    player: i32,
    marble: i32,
    row: &mut Vec<i32>,
    pos: i32,
    scores: &mut HashMap<i32, i32>,
) -> i32 {
    let n = row.len() as i32;
    let npos;

    if marble % 23 != 0 {
        npos = ((pos + 1) % n + 1) % (n + 1);
        row.insert(npos as usize, marble);
    } else {
        let to_del = if (pos - 7) > 0 { pos - 7 } else { n + pos - 7 };
        let del_value = row[to_del as usize];
        row.remove(to_del as usize);
        npos = to_del % n;
        let player_score = scores.entry(player).or_insert(0);
        *player_score += del_value + marble;
    }

    npos
}

fn play(players: i32, turns: i32) -> HashMap<i32, i32> {
    let mut scores: HashMap<i32, i32> = HashMap::new();

    let mut current_row: Vec<i32> = vec![0];
    let mut current_pos = 0;

    for marble in 1..=turns {
        let player = marble % players;
        current_pos = update_row(player, marble, &mut current_row, current_pos, &mut scores);
    }

    scores
}

fn max_score(players: i32, turns: i32) -> Option<i32> {
    let scores = play(players, turns);
    scores.values().max().map(|&n| n)
}

pub fn day9(input: &str) {
    // let mut file = File::open(input).expect("Failed to open input file");
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)
    //     .expect("Failed to read file");

    println!("Part 1: {:?}", max_score(470, 72170));
    println!("Part 2: {:?}", max_score(470, 72170 * 100));
}

#[test]
fn test_max_score() {
    assert_eq!(max_score(9, 25), Some(32));
    assert_eq!(max_score(10, 1618), Some(8317));
    assert_eq!(max_score(13, 7999), Some(146373));
    assert_eq!(max_score(17, 1104), Some(2764));
    assert_eq!(max_score(21, 6111), Some(54718));
    assert_eq!(max_score(30, 5807), Some(37305));
}
