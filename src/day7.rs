use regex::Regex;
use std::cmp::{Ord, Ordering};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error + Send + Sync>>;

type TaskName = char;

#[derive(Debug)]
struct TaskMap {
    tmap: HashMap<TaskName, HashSet<TaskName>>,
}

#[derive(PartialEq)]
struct Dependency {
    task: TaskName,
    depends_on: TaskName,
}

impl fmt::Debug for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} => {}", self.task, self.depends_on)
    }
}

impl Dependency {
    fn new(task: TaskName, depends_on: TaskName) -> Self {
        Dependency { task, depends_on }
    }

    fn parse(line: &str) -> Result<Dependency> {
        // Step C must be finished before step A can begin.
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Step (?P<parent>[A-Z]) must be finished before step (?P<task>[A-Z]) can begin\."
            )
            .unwrap();
        }

        match RE.captures(line) {
            Some(caps) => {
                let depends_on = caps["parent"].parse::<TaskName>()?;
                let task = caps["task"].parse::<TaskName>()?;

                let dep = Dependency { task, depends_on };

                Ok(dep)
            }
            None => Err(From::from(format!("Not a task spec: {}", line))),
        }
    }
}

impl TaskMap {
    fn empty() -> Self {
        TaskMap {
            tmap: HashMap::new(),
        }
    }

    fn add(&mut self, dep: Dependency) {
        {
            let v = self.tmap.entry(dep.task).or_default();
            v.insert(dep.depends_on);
        }
        {
            // Add an entry for depends_on as a task as well
            self.tmap.entry(dep.depends_on).or_default();
        }
    }

    // TODO: Add logic to check whether it is actually completable or not
    fn complete_task(&mut self, task: TaskName) -> bool {
        if self.tmap.contains_key(&task) {
            // Remove task entry from the tmap, and then remove task from the
            // dependency sets of other existing tasks

            self.tmap.remove(&task);
            for ds in self.tmap.values_mut() {
                ds.remove(&task);
            }

            true
        } else {
            false
        }
    }

    fn completable_tasks(&self) -> Vec<TaskName> {
        let mut ts = vec![];

        for (&t, ds) in &self.tmap {
            if ds.is_empty() {
                ts.push(t);
            }
        }

        ts
    }
}

fn process_tasks(mut tasks: TaskMap) -> Vec<TaskName> {
    let mut completion_list = vec![];
    loop {
        let mut ts = tasks.completable_tasks();

        if ts.is_empty() {
            break;
        }

        ts.sort();
        ts.reverse();

        let task = ts.pop().unwrap();

        if tasks.complete_task(task) {
            completion_list.push(task);
        } else {
        }
    }

    completion_list
}

pub fn day7(input: &str) {
    let f = File::open(input).expect("Failed to open input file");
    let reader = BufReader::new(f);

    let mut tasks = TaskMap::empty();

    for line in reader.lines() {
        let trecord = line.expect("failed to read input line");

        if let Ok(dep) = Dependency::parse(&trecord) {
            tasks.add(dep);
        } else {
            println!("Failed to parse line: {}", trecord);
        }
    }

    let cstring: String = process_tasks(tasks).iter().collect();

    println!("Completion order: {:?}", cstring);
}

#[test]
fn test_dependency_parse() {
    if let Ok(dep) = Dependency::parse("Step C must be finished before step A can begin.") {
        assert_eq!(
            dep,
            Dependency {
                task: 'A',
                depends_on: 'C'
            }
        );
    } else {
        assert!(false);
    }

    if let Ok(dep) = Dependency::parse("Step C must be finished before step F can begin.") {
        assert_eq!(
            dep,
            Dependency {
                task: 'F',
                depends_on: 'C'
            }
        );
    } else {
        assert!(false);
    }
}

#[test]
fn test_process_tasks() {
    let mut tasks = TaskMap::empty();
    tasks.add(Dependency::new('A', 'C'));
    tasks.add(Dependency::new('F', 'C'));
    tasks.add(Dependency::new('B', 'A'));
    tasks.add(Dependency::new('D', 'A'));
    tasks.add(Dependency::new('E', 'B'));
    tasks.add(Dependency::new('E', 'D'));
    tasks.add(Dependency::new('E', 'F'));

    println!("Task map: {:?}", tasks);

    let clist = process_tasks(tasks);

    assert_eq!(clist.iter().collect::<String>(), "CABDFE");
}
