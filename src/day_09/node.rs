use std::{str::FromStr, num::ParseIntError};
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Node {
    pub name: String,
    pub target: String,
    pub cost: usize,
}

impl Node {
    pub fn new_reversed(node: &Node) -> Node {
        Node {
            name: node.target.to_string(),
            target: node.name.to_string(),
            cost: node.cost
        }
    }
}

impl FromStr for Node {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spl = s.split(' ');
        let name = spl.next().unwrap().to_string();
        let target = spl.nth(1).unwrap().to_string();
        let cost = spl.nth(1).unwrap().parse::<usize>()?;
        Ok(Node {
            name,
            target,
            cost
        })
    }
}