use std::{str::FromStr, num::ParseIntError};

pub enum Operator {
    Read(usize, String),
    Not(String, String),
    Or(String, String, String),
    And(String, String, String),
    Rshift(String, usize, String),
    Lshift(String, usize, String),
}

impl FromStr for Operator {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        match (parts.next().unwrap(), parts.next().unwrap(), parts.next().unwrap(), parts.last()) {
            ("NOT", input, _, Some(output)) => Ok(Operator::Not(input.to_string(), output.to_string())),
            (input, "OR", input2, Some(output)) => Ok(Operator::Or(input.to_string(), input2.to_string(), output.to_string())),
            (input, "AND", input2, Some(output)) => Ok(Operator::And(input.to_string(), input2.to_string(), output.to_string())),
            (input, "RSHIFT", input2, Some(output)) => Ok(Operator::Rshift(input.to_string(), input2.parse::<usize>()?, output.to_string())),
            (input, "LSHIFT", input2, Some(output)) => Ok(Operator::Lshift(input.to_string(), input2.parse::<usize>()?, output.to_string())),
            (x, "->", output, _) => Ok(Operator::Read(x.parse::<usize>()?, output.to_string())),
            _ => panic!("Unknown command: {s}")
        }
    }
}