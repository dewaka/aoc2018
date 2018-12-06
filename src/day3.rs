// https://adventofcode.com/2018/day/3

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error + Send + Sync>>;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Sheet {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

impl Sheet {
    fn from(pos: (i32, i32), dim: (i32, i32)) -> Sheet {
        let (left, top) = pos;
        let (width, height) = dim;
        Sheet {
            left,
            top,
            width,
            height,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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
            None => Err(From::from(format!("Failed to parse claim from: {}", cstr))),
        }
    }
}

fn get_coords_map(claims: &Vec<Claim>) -> HashMap<(i32, i32), i32> {
    let mut coords: HashMap<(i32, i32), i32> = HashMap::new();

    // for each claim update the coords to keep track of used positions
    for c in claims {
        for x in c.sheet.left..c.sheet.left + c.sheet.width {
            for y in c.sheet.top..c.sheet.top + c.sheet.height {
                let pos = (x, y);

                let count = coords.entry(pos).or_insert(0);
                *count += 1;
            }
        }
    }

    coords
}

fn overlapping_area(coords_map: &HashMap<(i32, i32), i32>) -> i32 {
    // We only need to count posistions which are used more than once (overlapping positions)
    coords_map
        .values()
        .map(|&c| if c > 1 { 1 } else { 0 })
        .fold(0, |sum, c| sum + c)
}

// Will find the first non-overlapping Claim
fn find_non_overlapping(
    claims: &Vec<Claim>,
    coords_map: &HashMap<(i32, i32), i32>,
) -> Option<Claim> {
    for c in claims {
        let mut ok = true;

        for x in c.sheet.left..c.sheet.left + c.sheet.width {
            for y in c.sheet.top..c.sheet.top + c.sheet.height {
                let pos = (x, y);

                if let Some(&count) = coords_map.get(&pos) {
                    if count > 1 {
                        ok = false;
                        break;
                    }
                }
            }
        }

        if ok {
            return Some(c.clone());
        }
    }

    None
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

    let coords_map = get_coords_map(&claims);
    let area = overlapping_area(&coords_map);
    println!("Overlapping area: {}", area);

    // Part 2
    let result = find_non_overlapping(&claims, &coords_map);
    println!("Non overlapping claim: {:?}", result);
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
