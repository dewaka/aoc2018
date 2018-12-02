extern crate chrono;
extern crate clap;

use chrono::prelude::*;
use clap::{App, Arg};

mod day1;
mod day2;

fn valid_day(day: u32) -> bool {
    day >= 1 && day <= 31
}

fn solution_for(day: u32) {
    assert!(valid_day(day));

    let input_file = &format!("data/input{}", day);

    match day {
        1 => day1::day1(input_file),
        2 => day2::day2(input_file),
        n => println!("Day {} is not available", n),
    }
}

fn advent_of_code_for_today() -> u32 {
    Local::today().day()
}

fn main() {
    let matches = App::new("advent of rust 2018")
        .version("1.0")
        .author("Chathura Colombage")
        .about("advent of code solutions for 2018 edition")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .help("run solution for the given day")
                .takes_value(true)
                .required(false),
        ).get_matches();

    let day: u32 = matches
        .value_of("day")
        .unwrap_or("0")
        .parse()
        .expect("Invalid day!");

    if day == 0 {
        let today = advent_of_code_for_today();
        solution_for(today);
    } else {
        if valid_day(day) {
            solution_for(day);
        } else {
            println!("Invalid day: {}", day);
        }
    }
}
