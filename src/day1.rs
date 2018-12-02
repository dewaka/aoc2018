// https://adventofcode.com/2018/day/1

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
struct Frequency(i32);

impl Frequency {
    fn new() -> Self {
        Frequency(0)
    }

    fn change(self, c: i32) -> Self {
        let Frequency(x) = self;
        Frequency(x + c)
    }

    fn add_changes(self, changes: &[i32]) -> Self {
        changes.iter().fold(self, |f, y| f.change(*y))
    }

    fn add_change_from_str(self, cstr: &str) -> Self {
        let num = cstr
            .trim()
            .parse::<i32>()
            .expect(&format!("Failed to parse change: {}", cstr));

        self.change(num)
    }

    fn add_changes_from_str(self, cstrs: &[&str]) -> Self {
        cstrs
            .iter()
            .fold(self, |f, cstr| f.add_change_from_str(&cstr))
    }

    fn value(&self) -> i32 {
        let &Frequency(num) = self;
        num
    }

    fn first_repeating_value(&self, changes: &[i32]) -> i32 {
        let mut seen = HashMap::new();
        let mut freq = Frequency(self.value());

        loop {
            for c in changes {
                let value = freq.value();

                let count = seen.entry(value).or_insert(1);
                if *count == 2 {
                    return value;
                }
                *count += 1;

                freq = freq.change(*c);
            }
        }
    }
}

fn changes_from_file(input: &str) -> Vec<i32> {
    let f = File::open(input).expect("file not found");
    let file = BufReader::new(&f);

    file.lines()
        .map(|line| {
            let change = line.expect("could not read line");
            change
                .parse::<i32>()
                .expect(&format!("could not parse change: {}", change))
        }).collect()
}

fn day1_part2(changes: &[i32]) {
    let result = Frequency::new().first_repeating_value(changes);
    println!("First repeating value: {}", result);
}

pub fn day1(input: &str) {
    let changes = changes_from_file(input);
    let Frequency(num) = Frequency::new().add_changes(&changes);
    println!("Final frequency: {}", num);

    day1_part2(&changes);
}

#[test]
fn test_add_changes() {
    assert_eq!(Frequency::new().add_changes(&vec!(1, 1, 1)), Frequency(3));
    assert_eq!(Frequency::new().add_changes(&vec!(1, 1, -2)), Frequency(0));
    assert_eq!(
        Frequency::new().add_changes(&vec!(-1, -2, -3)),
        Frequency(-6)
    );
}

#[test]
fn test_add_change_from_str() {
    assert_eq!(Frequency::new().add_change_from_str("+1"), Frequency(1));
    assert_eq!(Frequency::new().add_change_from_str(" +10 "), Frequency(10));
    assert_eq!(Frequency::new().add_change_from_str("-7"), Frequency(-7));
    assert_eq!(
        Frequency::new().add_change_from_str("   -68  "),
        Frequency(-68)
    );
}

#[test]
fn test_change_frequency_from_strs() {
    assert_eq!(
        Frequency::new().add_changes_from_str(&"+1".split(",").collect::<Vec<&str>>()),
        Frequency(1)
    );

    assert_eq!(
        Frequency::new().add_changes_from_str(&"+1, +1, +1".split(",").collect::<Vec<&str>>()),
        Frequency(3)
    );

    assert_eq!(
        Frequency::new().add_changes_from_str(&"+1, +1, -2".split(",").collect::<Vec<&str>>()),
        Frequency(0)
    );

    assert_eq!(
        Frequency::new().add_changes_from_str(&"-1, -2, -3".split(",").collect::<Vec<&str>>()),
        Frequency(-6)
    );
}

#[test]
fn test_first_repeating_value() {
    assert_eq!(Frequency::new().first_repeating_value(&vec!(1, -1)), 0);
    assert_eq!(Frequency::new().first_repeating_value(&vec!(3, 3, 4, -2, -4)), 10);
    assert_eq!(Frequency::new().first_repeating_value(&vec!(-6, 3, 8, 5, -6)), 5);
    assert_eq!(Frequency::new().first_repeating_value(&vec!(7, 7, -2, -7, -4)), 14);
}
