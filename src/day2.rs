// https://adventofcode.com/2018/day/2

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Counts = (i32, i32);

fn frequency_map(id: &str) -> HashMap<char, i32> {
    let mut fmap = HashMap::new();

    for c in id.chars() {
        let count = fmap.entry(c).or_default();
        *count += 1;
    }

    fmap
}

fn get_counts_for(id: &str) -> Counts {
    let fmap = frequency_map(id);
    let mut twos = false;
    let mut threes = false;

    for (_, &c) in &fmap {
        if c == 2 {
            twos = true;
        } else if c == 3 {
            threes = true;
        }
    }

    let two_count = if twos { 1 } else { 0 };
    let three_count = if threes { 1 } else { 0 };

    let res = (two_count, three_count);
    res
}

fn checksum(ids: &[String]) -> i32 {
    let (sum2, sum3) = ids
        .iter()
        .map(|x| get_counts_for(x))
        .fold((0, 0), |(c2, c3), (x, y)| (c2 + x, c3 + y));

    sum2 * sum3
}

pub fn day2(input: &str) {
    let f = File::open(input).expect("file not found");
    let file = BufReader::new(&f);

    let ids: Vec<String> = file
        .lines()
        .map(|line| line.expect("fail to read input line").to_string())
        .collect();

    let csum = checksum(&ids);

    println!("Checksum: {}", csum);
}

#[test]
fn test() {
    assert_eq!(get_counts_for("abcdef"), (0, 0));
    assert_eq!(get_counts_for("bababc"), (1, 1));
    assert_eq!(get_counts_for("abbcde"), (1, 0));
    assert_eq!(get_counts_for("abcccd"), (0, 1));
    assert_eq!(get_counts_for("aabcdd"), (1, 0));
    assert_eq!(get_counts_for("abcdee"), (1, 0));
    assert_eq!(get_counts_for("ababab"), (0, 1));
}

#[test]
fn test_checksum() {
    let ids = vec![
        "abcdef".to_owned(),
        "bababc".to_owned(),
        "abbcde".to_owned(),
        "abcccd".to_owned(),
        "aabcdd".to_owned(),
        "abcdee".to_owned(),
        "ababab".to_owned(),
    ];

    assert_eq!(checksum(&ids), 12);
}
