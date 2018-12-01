// https://adventofcode.com/2018/day/1

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
struct Frequency(i32);

impl Frequency {
    fn new() -> Frequency {
        Frequency(0)
    }

    fn change(self, c: i32) -> Frequency {
        let Frequency(x) = self;
        Frequency(x + c)
    }

    fn add_changes(self, changes: &[i32]) -> Frequency {
        changes.iter().fold(self, |f, y| f.change(*y))
    }

    fn add_change_from_str(self, cstr: &str) -> Frequency {
        match cstr.trim().parse::<i32>() {
            Ok(num) => self.change(num),
            Err(_) => self,
        }
    }

    fn add_changes_from_str(self, cstrs: &[&str]) -> Frequency {
        cstrs
            .iter()
            .fold(self, |f, cstr| f.add_change_from_str(&cstr))
    }
}

pub fn day1(input: &str) {
    let f = File::open(input).expect("file not found");
    let file = BufReader::new(&f);

    let mut freq = Frequency::new();

    for line in file.lines() {
        let change = line.expect("could not read line");
        freq = freq.add_change_from_str(&change);
    }

    let Frequency(num) = freq;
    println!("Final frequency: {}", num);
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
