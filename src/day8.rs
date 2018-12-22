use std::fs::File;
use std::iter::Iterator;
use std::io::prelude::*;

#[derive(Debug)]
struct Node {
    children: Vec<Box<Node>>,
    metadata: Vec<i32>,
}

impl Node {
    fn new(children: Vec<Box<Node>>, metadata: Vec<i32>) -> Node {
        Node { children, metadata }
    }

    fn from(values: &[i32]) -> Self {
        Node::get_node(&mut values.iter())
    }

    fn get_node(values: &mut Iterator<Item = &i32>) -> Self {
        let nc = *values.next().unwrap();
        let nm = *values.next().unwrap();

        let mut children = vec![];
        for _ in 0..nc {
            children.push(Box::new(Node::get_node(values)));
        }

        let mut metadata = vec![];
        for _ in 0..nm {
            metadata.push(*values.next().unwrap());
        }

        Node::new(children, metadata)
    }

    fn sum_metadata(&self) -> i32 {
        // node metadata sum
        let msum: i32 = self.metadata.iter().sum();

        // metadata sum of children
        let csum: i32 = self.children.iter().map(|c| c.sum_metadata()).sum();

        // total
        msum + csum
    }
}

pub fn day8(input: &str) {
    let mut file = File::open(input).expect("Failed to open input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let values = contents.trim().split(" ").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let node = Node::from(&values);

    println!("Sum metadata: {}", node.sum_metadata());
}

#[test]
fn test_sum() {
    let node = Node::from(&vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2]);
    assert_eq!(node.sum_metadata(), 138);
}
