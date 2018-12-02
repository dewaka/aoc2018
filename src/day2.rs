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

fn get_counts(id: &str) -> Counts {
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
        .map(|x| get_counts(x))
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
    day2_part2(&ids);
}

fn day2_part2(ids: &[String]) {
    match common_letters_in_ids(ids) {
        Some(letters) => println!("Common letters are: {}", letters),
        None => println!("No common letters found!"),
    }
}

fn distance(left: &str, right: &str) -> i32 {
    left.chars()
        .zip(right.chars())
        .map(|(x, y)| if x == y { 0 } else { 1 })
        .fold(0, |acc, x| acc + x)
}

fn correct_boxes(ids: &[String]) -> Option<(String, String)> {
    for i in 0..ids.len() - 1 {
        for j in i + 1..ids.len() {
            let left = &ids[i];
            let right = &ids[j];

            if distance(left, right) == 1 {
                return Some((left.to_owned(), right.to_owned()));
            }
        }
    }

    None
}

fn same_letters(left: &str, right: &str) -> String {
    let mut same = String::new();

    for (l, r) in left.chars().zip(right.chars()) {
        if l == r {
            same.push(l);
        }
    }

    same
}

fn common_letters_in_ids(ids: &[String]) -> Option<String> {
    match correct_boxes(ids) {
        Some((left, right)) => Some(same_letters(&left, &right)),
        None => None,
    }
}

#[test]
fn test_get_counts() {
    assert_eq!(get_counts("abcdef"), (0, 0));
    assert_eq!(get_counts("bababc"), (1, 1));
    assert_eq!(get_counts("abbcde"), (1, 0));
    assert_eq!(get_counts("abcccd"), (0, 1));
    assert_eq!(get_counts("aabcdd"), (1, 0));
    assert_eq!(get_counts("abcdee"), (1, 0));
    assert_eq!(get_counts("ababab"), (0, 1));
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

#[test]
fn test_distance() {
    assert_eq!(distance("abcde", "axcye"), 2);
    assert_eq!(distance("fghij", "fguij"), 1);
}

#[test]
fn test_correct_boxes() {
    let ids = vec![
        "abcde".to_owned(),
        "fghij".to_owned(),
        "klmno".to_owned(),
        "pqrst".to_owned(),
        "fguij".to_owned(),
        "axcye".to_owned(),
        "wvxyz".to_owned(),
    ];

    assert_eq!(
        correct_boxes(&ids),
        Some(("fghij".to_owned(), "fguij".to_owned()))
    );
}

#[test]
fn test_common_letters_in_ids() {
    let ids = vec![
        "abcde".to_owned(),
        "fghij".to_owned(),
        "klmno".to_owned(),
        "pqrst".to_owned(),
        "fguij".to_owned(),
        "axcye".to_owned(),
        "wvxyz".to_owned(),
    ];

    assert_eq!(common_letters_in_ids(&ids), Some("fgij".to_owned()));
}
