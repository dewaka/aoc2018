// To find most minutes sleep guard is just accumulation of minutes over the given records
// But is it just that?

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use chrono::prelude::*;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error + Send + Sync>>;

#[derive(Debug, PartialEq)]
enum Action {
    Shift { guard: i32 },
    Sleep,
    WakeUp,
}

#[derive(Debug, PartialEq)]
struct Record {
    time: DateTime<Utc>,
    observation: Action,
}

// What do we need to keep stats of about the guards?
// 1. We need to keep the total time they sleep
// 2. Then we also need to know which minute of the hour they are most likely to fall asleep
#[derive(Debug)]
struct Stats {
    sleep_total: i32,
    sleep_times: HashMap<i32, i32>,
}

impl Stats {
    fn record(&mut self, start: DateTime<Utc>, duration: i32) {
        self.sleep_total += duration;
        let start_minute = start.minute();

        for m in start_minute as i32..start_minute as i32 + duration {
            let count = self.sleep_times.entry(m).or_insert(0);
            *count += 1;
        }
    }

    fn empty() -> Self {
        Stats {
            sleep_total: 0,
            sleep_times: HashMap::new(),
        }
    }

    // Returns the most frequently slept minute and the times it was found to be
    fn get_most_frequent(&self) -> Option<(&i32, &i32)> {
        self.sleep_times.iter().max_by(|a, b| a.1.cmp(b.1))
    }
}

impl Record {
    fn parse(srecord: &str) -> Result<Record> {
        if srecord.ends_with("begins shift") {
            Self::parse_shift_record(srecord)
        } else if srecord.ends_with("falls asleep") {
            Self::parse_sleep_record(srecord)
        } else if srecord.ends_with("wakes up") {
            Self::parse_wakeup_record(srecord)
        } else {
            Err(From::from(format!("Failed to parse record: {}", srecord)))
        }
    }

    fn parse_shift_record(srecord: &str) -> Result<Record> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"\[(?P<date>.*)\] Guard #(?P<id>\d+) begins shift").unwrap();
        }

        match RE.captures(srecord) {
            Some(caps) => {
                let date = Utc.datetime_from_str(&caps["date"], "%Y-%m-%d %H:%M")?;
                let id = caps["id"].parse::<i32>()?;

                let record = Record {
                    time: date,
                    observation: Action::Shift { guard: id },
                };

                Ok(record)
            }
            None => Err(From::from(format!("Not a valid shift record: {}", srecord))),
        }
    }

    fn parse_sleep_record(srecord: &str) -> Result<Record> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[(?P<date>.*)\] falls asleep").unwrap();
        }

        match RE.captures(srecord) {
            Some(caps) => {
                let date = Utc.datetime_from_str(&caps["date"], "%Y-%m-%d %H:%M")?;

                let record = Record {
                    time: date,
                    observation: Action::Sleep,
                };

                Ok(record)
            }
            None => Err(From::from(format!("Not a valid sleep record: {}", srecord))),
        }
    }

    fn parse_wakeup_record(srecord: &str) -> Result<Record> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[(?P<date>.*)\] wakes up").unwrap();
        }

        match RE.captures(srecord) {
            Some(caps) => {
                let date = Utc.datetime_from_str(&caps["date"], "%Y-%m-%d %H:%M")?;

                let record = Record {
                    time: date,
                    observation: Action::WakeUp,
                };

                Ok(record)
            }
            None => Err(From::from(format!(
                "Not a valid wake up record: {}",
                srecord
            ))),
        }
    }

    // Sort records by the timestamp in the ascending order
    fn sort_records(rs: &mut Vec<Record>) {
        rs.sort_by(|a, b| a.time.cmp(&b.time));
    }

    fn process(records: &Vec<Record>) -> HashMap<i32, Stats> {
        // Go through the records and process the sleep times of Guards
        let mut current_guard = None;
        let mut sleep_time = None;

        let mut stats = HashMap::new();

        for r in records {
            match r.observation {
                Action::Shift { guard } => current_guard = Some(guard),
                Action::Sleep => sleep_time = Some(r.time),
                Action::WakeUp => {
                    if let Some(st) = sleep_time {
                        let duration = (r.time.minute() - st.minute()) as i32;
                        let id =
                            current_guard.expect("Invalid record - no guard to record time for");
                        let stat = stats.entry(id).or_insert(Stats::empty());
                        stat.record(st, duration);
                    } else {
                        println!("Error - sleep time not recorded!");
                    }
                }
            }
        }

        stats
    }

    fn find_best_by_total(stats: &HashMap<i32, Stats>) {
        // We need to find who sleeps the most
        let m = stats
            .iter()
            .map(|(id, st)| (id, st.sleep_total))
            .max_by(|x, y| x.1.cmp(&y.1));

        if let Some((guard, total)) = m {
            println!("Guard {} sleeps most with total {} minutes.", guard, total);

            // Then we need to find the minute they like to sleep the most
            let stat_for_guard = stats.get(guard).unwrap();
            let (frequent_min, _) = stat_for_guard.get_most_frequent().unwrap();
            println!("Most frequent minute: {:?}", frequent_min);

            let answer = frequent_min * guard;
            println!("Best by total number of minutes: {}", answer);
        } else {
            println!("Error: could not find the guard who sleeps most!");
        }
    }

    fn find_best_by_most(stats: &HashMap<i32, Stats>) {
        let max_by_minute_times = stats
            .iter()
            .map(|(id, s)| {
                let (minute, times) = s.get_most_frequent().unwrap();
                (id, times, minute)
            }).max_by(|a, b| a.1.cmp(&b.1));

        let (guard, _, minute) = max_by_minute_times.unwrap();
        let answer = guard * minute;
        println!("Best by most minutes: {}", answer);
    }
}

pub fn day4(input: &str) {
    let f = File::open(input).expect("file not found");
    let file = BufReader::new(&f);

    let mut records = vec![];

    for line in file.lines() {
        let srecord = line.expect("failed to read input line");

        if let Ok(record) = Record::parse(&srecord) {
            records.push(record);
        } else {
            println!("Failed to parse line: {}", srecord);
        }
    }

    Record::sort_records(&mut records);
    let records = Record::process(&records);
    Record::find_best_by_total(&records);
    Record::find_best_by_most(&records);
}

#[test]
fn test_parse_date_format() {
    let d = Utc
        .datetime_from_str("1518-11-01 00:00", "%Y-%m-%d %H:%M")
        .unwrap();
    assert_eq!(d.year(), 1518);
    assert_eq!(d.month(), 11);
    assert_eq!(d.day(), 1);
    assert_eq!(d.hour(), 0);
    assert_eq!(d.minute(), 0);

    let d = Utc
        .datetime_from_str("1518-11-04 00:36", "%Y-%m-%d %H:%M")
        .unwrap();
    assert_eq!(d.year(), 1518);
    assert_eq!(d.month(), 11);
    assert_eq!(d.day(), 4);
    assert_eq!(d.hour(), 0);
    assert_eq!(d.minute(), 36);
}

#[test]
fn test_parse_record() {
    let record = Record::parse("[1518-11-01 00:00] Guard #10 begins shift").unwrap();
    assert_eq!(record.time.year(), 1518);
    assert_eq!(record.time.month(), 11);
    assert_eq!(record.time.day(), 1);
    assert_eq!(record.observation, Action::Shift { guard: 10 });

    let record = Record::parse("[1518-7-8 00:05] falls asleep").unwrap();
    assert_eq!(record.time.year(), 1518);
    assert_eq!(record.time.month(), 7);
    assert_eq!(record.time.day(), 8);
    assert_eq!(record.observation, Action::Sleep);

    let record = Record::parse("[1518-11-30 00:25] wakes up").unwrap();
    assert_eq!(record.time.year(), 1518);
    assert_eq!(record.time.month(), 11);
    assert_eq!(record.time.day(), 30);
    assert_eq!(record.observation, Action::WakeUp);
}
