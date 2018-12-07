// https://adventofcode.com/2018/day/5

use std::fs::File;
use std::io::{BufRead, BufReader};

fn reacts(x: char, y: char) -> bool {
    (x != y) && (x.to_lowercase().to_string() == y.to_lowercase().to_string())
}

fn polymer_react(ps: &str) -> String {
    let mut poly: Vec<char> = vec![];

    for c in ps.chars() {
        match poly.last() {
            Some(&t) => {
                if reacts(t, c) {
                    poly.pop();
                } else {
                    poly.push(c);
                }
            }
            None => poly.push(c),
        }
    }

    poly.iter().collect()
}

pub fn day5(input: &str) {
    let f = File::open(input).expect("Failed to open input file");
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read buffer");

    let polymer = line.trim();
    println!("Polymer1 length: {}", polymer_react(polymer).len());

    if let Some(shortest) = shortest_polymer(polymer) {
        println!("Shortest polymer length: {}", shortest.len());
    } else {
        println!("Couldn't find the shortest polymer");
    }
}

fn filter_unit(ps: &str, u: char) -> String {
    ps.chars()
        .filter(|&c| c.to_lowercase().to_string() != u.to_lowercase().to_string())
        .collect()
}

fn shortest_polymer(ps: &str) -> Option<String> {
    ('a' as u8..('z' as u8 + 1))
        .map(|u| polymer_react(&filter_unit(ps, u as char)))
        .min_by_key(|x| x.len())
}

#[test]
fn test_polymer_react() {
    assert_eq!(polymer_react("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
    assert_eq!(polymer_react("aaA"), "a");
    assert_eq!(polymer_react("aAa"), "a");
    assert_eq!(polymer_react("aAab"), "ab");
    assert_eq!(polymer_react("cAaaC"), "caC");
    assert_eq!(polymer_react("cCAdDbEeC"), "AbC");
}

#[test]
fn test_polymer_filter() {
    assert_eq!(filter_unit("aaA", 'a'), "");
    assert_eq!(filter_unit("aaA", 'A'), "");
    assert_eq!(filter_unit("aaAbdc", 'a'), "bdc");
    assert_eq!(filter_unit("aaAbdc", 'A'), "bdc");
    assert_eq!(filter_unit("aaAbdc", 'b'), "aaAdc");
}
