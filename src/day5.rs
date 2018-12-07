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

    line.trim();

    println!("Polymer1 length: {}", polymer_react(&line).len());
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
