#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate clap;
extern crate regex;

use chrono::prelude::*;
use clap::{App, Arg};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn valid_day(day: u32) -> bool {
    day >= 1 && day <= 31
}

fn solution_for(day: u32) {
    assert!(valid_day(day));

    let input_file = &format!("data/input{}", day);

    match day {
        1 => day1::day1(input_file),
        2 => day2::day2(input_file),
        3 => day3::day3(input_file),
        4 => day4::day4(input_file),
        5 => day5::day5(input_file),
        6 => day6::day6(input_file),
        7 => day7::day7(input_file),
        8 => day8::day8(input_file),
        9 => day9::day9(input_file),
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
        )
        .get_matches();

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
