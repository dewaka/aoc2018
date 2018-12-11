use std::cmp::{Ord, Ordering};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

type Point = (i32, i32);

fn manhatten_distance(x: Point, y: Point) -> i32 {
    (x.0 - y.0).abs() + (x.1 - y.1).abs()
}

fn find_closest(p: Point, points: &Vec<Point>) -> Option<Point> {
    unique_extreme_by_iter(
        &mut points.iter().map(|&s| (s, manhatten_distance(p, s))),
        |x, y| x.1.cmp(&y.1),
    )
    .map(|x| x.0)
}

fn unique_extreme_by_iter<T, F>(stuff: &mut Iterator<Item = T>, compare: F) -> Option<T>
where
    T: Copy,
    F: Fn(T, T) -> Ordering,
{
    let mut umin = if let Some(n) = stuff.next() {
        Some(n)
    } else {
        None
    };

    let mut min = umin;

    while let Some(x) = stuff.next() {
        if let Some(m) = min {
            let c = compare(x, m);

            if c == Ordering::Less {
                min = Some(x);
                umin = Some(x);
            } else if c == Ordering::Equal {
                umin = None;
            }
        }
    }

    umin
}

struct Bounds {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

fn score_around(points: &Vec<Point>, bounds: &Bounds, out: i32) -> HashMap<Point, i32> {
    let mut point_scores = HashMap::new();
    for i in bounds.min_x - out..bounds.max_x + out + 1 {
        for j in bounds.min_y - out..bounds.max_y + out + 1 {
            let p = (i, j);

            if let Some(closest) = find_closest(p, points) {
                let count = point_scores.entry(closest).or_default();
                *count += 1;
            }
        }
    }

    point_scores
}

fn total_distance_to_point(p: Point, points: &Vec<Point>) -> i32 {
    points
        .iter()
        .fold(0, |acc, &t| acc + manhatten_distance(p, t))
}

fn points_around_within_distance(
    points: &Vec<Point>,
    bounds: &Bounds,
    distance: i32,
    around: i32,
) -> HashSet<Point> {
    let mut region = HashSet::new();

    for i in bounds.min_x - around..bounds.max_x + around + 1 {
        for j in bounds.min_y - around..bounds.max_y + around + 1 {
            let p = (i, j);

            if total_distance_to_point(p, points) < distance {
                region.insert(p);
            }
        }
    }

    region
}

fn process_distance(points: &Vec<Point>, bounds: &Bounds) -> Option<i32> {
    let s1 = score_around(points, bounds, 400);
    let s2 = score_around(points, bounds, 600);

    let mut bounded: Vec<(Point, i32)> = vec![];

    for (&p, &d) in &s1 {
        if let Some(&d2) = s2.get(&p) {
            if d == d2 {
                bounded.push((p, d));
            }
        }
    }

    bounded.sort_by_key(|k| k.1);

    bounded.last().map(|k| k.1)
}

fn bounds_of(points: &Vec<Point>) -> Bounds {
    let min_x: i32 = points
        .iter()
        .map(|x| x.0)
        .min()
        .expect("Couldn't find min x");
    let max_x: i32 = points
        .iter()
        .map(|x| x.0)
        .max()
        .expect("Couldn't find max x");

    let min_y: i32 = points
        .iter()
        .map(|x| x.1)
        .min()
        .expect("Couldn't find min y");
    let max_y: i32 = points
        .iter()
        .map(|x| x.1)
        .max()
        .expect("Couldn't find max y");

    Bounds {
        min_x,
        max_x,
        min_y,
        max_y,
    }
}

pub fn day6(input: &str) {
    let f = File::open(input).expect("Failed to open input file");
    let reader = BufReader::new(f);
    let mut points = vec![];

    for line in reader.lines() {
        let spoint = line.expect("Failed to read line");
        let parts: Vec<&str> = spoint.split(",").collect();
        if parts.len() == 2 {
            let x = parts[0].trim().parse().unwrap();
            let y = parts[1].trim().parse().unwrap();
            points.push((x, y));
        }
    }

    let bounds = bounds_of(&points);

    let max_bounded = process_distance(&points, &bounds);
    println!("Max bounded: {:?}", max_bounded);

    let distance = 10000;
    let region_within = points_around_within_distance(&points, &bounds, distance, 400);

    println!("Bounded region contains {} points", region_within.len());
}

#[test]
fn test_manhatten_distance() {
    assert_eq!(manhatten_distance((1, 3), (3, 4)), 3);
}

#[test]
fn test_process_distance() {
    let points = vec![(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)];
    let bounds = bounds_of(&points);
    assert_eq!(process_distance(&points, &bounds), Some(17));
}
