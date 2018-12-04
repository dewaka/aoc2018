// https://adventofcode.com/2018/day/3

use regex::Regex;
use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Sheet {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

#[derive(Debug, PartialEq)]
struct Claim {
    id: i32,
    sheet: Sheet,
}

impl Claim {
    fn parse_claim(cstr: &str) -> Result<Claim> {
        let re = Regex::new(
            r"^#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)$",
        )?;

        match re.captures(cstr) {
            Some(caps) => {
                let id = caps["id"].parse::<i32>()?;
                let left = caps["left"].parse::<i32>()?;
                let top = caps["top"].parse::<i32>()?;
                let width = caps["width"].parse::<i32>()?;
                let height = caps["height"].parse::<i32>()?;

                let claim = Claim {
                    id,
                    sheet: Sheet {
                        left,
                        top,
                        width,
                        height,
                    },
                };

                Ok(claim)
            }
            None => Err(From::from("Failed to parse claim")),
        }
    }

    fn overlap(&self, other: &Self) -> Option<Sheet> {
        self.sheet.overlap(&other.sheet)
    }
}

impl Sheet {
    fn from(pos: (i32, i32), dim: (i32, i32)) -> Self {
        let (left, top) = pos;
        let (width, height) = dim;

        Sheet {
            left,
            top,
            width,
            height,
        }
    }

    fn width_overlap(&self, other: &Self) -> Option<(i32, i32)> {
        let (x1, x2) = (self.left, self.left + self.width);
        let (t1, t2) = (other.left, other.left + other.width);

        if x1 <= t1 && t1 < x2 {
            Some((t1, min(x2, t2) - t1))
        } else if t1 <= x1 && x1 < t2 {
            Some((x1, min(x2, t2) - x1))
        } else {
            None
        }
    }

    fn height_overlap(&self, other: &Self) -> Option<(i32, i32)> {
        let (y1, y2) = (self.top, self.top + self.height);
        let (t1, t2) = (other.top, other.top + other.height);

        if y1 <= t1 && t1 < y2 {
            Some((t1, min(y2, t2) - t1))
        } else if t1 <= y1 && y1 < t2 {
            Some((y1, min(y2, t2) - y1))
        } else {
            None
        }
    }

    fn overlap(&self, other: &Self) -> Option<Self> {
        match self.width_overlap(other) {
            Some((x, width)) => match self.height_overlap(other) {
                Some((y, height)) => Some(Self::from((x, y), (width, height))),
                None => None,
            },
            None => None,
        }
    }

    fn area(&self) -> i32 {
        self.width * self.height
    }
}

fn sheet_area_sum(sheets: &Vec<Sheet>) -> i32 {
    let mut total = 0;

    for i in 0..sheets.len() - 1 {
        let mut sum = sheets[i].area();

        for j in i + 1..sheets.len() {
            if let Some(s) = sheets[i].overlap(&sheets[j]) {
                sum -= s.area();
                if sum <= 0 {
                    break;
                }
            }
        }

        total += sum;
    }

    total
}

fn overlapping_area(claims: &Vec<Claim>) -> i32 {
    // let mut overlap_set: HashSet<Sheet> = HashSet::new();
    let mut sheets: Vec<Sheet> = vec!();

    for i in 0..claims.len() - 1 {
        for j in i + 1..claims.len() {
            let c1 = &claims[i];
            let c2 = &claims[j];

            if let Some(sheet) = c1.overlap(&c2) {
                // overlap_set.insert(sheet.clone());
                sheets.push(sheet.clone());
            }
        }
    }

    // for s in overlap_set.iter() {
    //     sheets.push(s.clone());
    // }

    sheet_area_sum(&sheets)
}

pub fn day3(input: &str) {
    let f = File::open(input).expect("file not found");
    let file = BufReader::new(&f);
    let mut claims: Vec<Claim> = vec![];

    for line in file.lines() {
        let cstr = line.expect("failed to read input line");
        if let Ok(claim) = Claim::parse_claim(&cstr) {
            claims.push(claim);
        } else {
            println!("Failed to parse line: {}", cstr);
        }
    }

    let area = overlapping_area(&claims);
    println!("Overlapping area: {}", area);
}

#[test]
fn test_width_overlap() {
    fn test_overlap(s1: &Sheet, s2: &Sheet, expected: Option<(i32, i32)>) {
        assert_eq!(s1.width_overlap(&s2), expected);
        assert_eq!(s2.width_overlap(&s1), expected);
    }

    test_overlap(
        &Sheet::from((2, 3), (5, 6)), // 2, 7
        &Sheet::from((3, 4), (2, 2)), // 3, 5
        Some((3, 2)),
    );

    test_overlap(
        &Sheet::from((1, 3), (4, 4)), // 1, 5
        &Sheet::from((3, 1), (4, 4)), // 3, 7
        Some((3, 2)),
    );

    test_overlap(
        &Sheet::from((1, 2), (4, 4)), // 1, 5
        &Sheet::from((1, 6), (1, 4)), // 1, 2
        Some((1, 1)),
    );

    test_overlap(
        &Sheet::from((1, 8), (8, 4)), // 1, 9
        &Sheet::from((8, 6), (1, 4)), // 8, 9
        Some((8, 1)),
    );
}

#[test]
fn test_height_overlap() {
    fn test_overlap(s1: &Sheet, s2: &Sheet, expected: Option<(i32, i32)>) {
        assert_eq!(s1.height_overlap(&s2), expected);
        assert_eq!(s2.height_overlap(&s1), expected);
    }

    test_overlap(
        &Sheet::from((2, 3), (5, 6)), // 3, 9
        &Sheet::from((3, 4), (2, 2)), // 4, 6
        Some((4, 2)),
    );

    test_overlap(
        &Sheet::from((1, 3), (4, 4)), // 3, 7
        &Sheet::from((3, 1), (4, 4)), // 1, 5
        Some((3, 2)),
    );

    test_overlap(
        &Sheet::from((1, 2), (4, 4)), // 2, 6
        &Sheet::from((1, 6), (1, 4)), // 6, 10
        None,
    );

    test_overlap(
        &Sheet::from((1, 1), (4, 5)), // 1, 6
        &Sheet::from((1, 5), (1, 1)), // 5, 6
        Some((5, 1)),
    );
}

#[test]
fn test_sheet_overlap() {
    fn test_overlap(s1: &Sheet, s2: &Sheet, expected: Option<Sheet>) {
        assert_eq!(s1.overlap(&s2), expected);
        assert_eq!(s2.overlap(&s1), expected);
    }

    test_overlap(
        &Sheet::from((1, 3), (4, 4)), // (1, 5), (3, 7)
        &Sheet::from((3, 1), (4, 4)), // (3, 7), (1, 5)
        // (3, 2) and (3, 2)
        Some(Sheet::from((3, 3), (2, 2))),
    );
}

#[test]
fn test_parse_claim() {
    fn test(cstr: &str, claim: Claim) {
        match Claim::parse_claim(cstr) {
            Ok(c) => assert_eq!(c, claim),
            Err(_) => assert!(false),
        }
    }

    test(
        "#19 @ 269,410: 23x11",
        Claim {
            id: 19,
            sheet: Sheet::from((269, 410), (23, 11)),
        },
    );

    test(
        "#20 @ 534,291: 13x11",
        Claim {
            id: 20,
            sheet: Sheet::from((534, 291), (13, 11)),
        },
    );

    test(
        "#21 @ 331,206: 22x28",
        Claim {
            id: 21,
            sheet: Sheet::from((331, 206), (22, 28)),
        },
    );

    test(
        "#22 @ 648,629: 20x19",
        Claim {
            id: 22,
            sheet: Sheet::from((648, 629), (20, 19)),
        },
    );

    test(
        "#23 @ 302,342: 24x12",
        Claim {
            id: 23,
            sheet: Sheet::from((302, 342), (24, 12)),
        },
    );
}

#[test]
fn test_overlapping_area() {
    let claims = vec![
        Claim {
            id: 1,
            sheet: Sheet::from((1, 3), (4, 4)),
        },
        Claim {
            id: 2,
            sheet: Sheet::from((3, 1), (4, 4)),
        },
    ];

    assert_eq!(overlapping_area(&claims), 4);
}
