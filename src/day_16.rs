use std::fs;

enum Property {
    Children(usize),
    Cats(usize),
    Samoyeds(usize),
    Pomeranians(usize),
    Akitas(usize),
    Vizslas(usize),
    Goldfish(usize),
    Trees(usize),
    Cars(usize),
    Perfumes(usize),
}

impl Property {
    fn from(name: String, value: usize) -> Property {
        match name.replace(":", "").as_str() {
            "children" => Property::Children(value),
            "cats" => Property::Cats(value),
            "samoyeds" => Property::Samoyeds(value),
            "pomeranians" => Property::Pomeranians(value),
            "akitas" => Property::Akitas(value),
            "vizslas" => Property::Vizslas(value),
            "goldfish" => Property::Goldfish(value),
            "trees" => Property::Trees(value),
            "cars" => Property::Cars(value),
            "perfumes" => Property::Perfumes(value),
            _ => panic!("Unknown property"),
        }
    }
    fn matches_expected(&self, use_exact_match: bool) -> bool {
        match self {
            Property::Children(x) => x == &3,
            Property::Cats(x) => if use_exact_match { x == &7 } else { x > &7 },
            Property::Samoyeds(x) => x == &2,
            Property::Pomeranians(x) => if use_exact_match { x == &3 } else { x < &3 },
            Property::Akitas(x) => x == &0,
            Property::Vizslas(x) => x == &0,
            Property::Goldfish(x) => if use_exact_match { x == &5 } else { x < &5 },
            Property::Trees(x) => if use_exact_match { x == &3 } else { x > &3 },
            Property::Cars(x) => x == &2,
            Property::Perfumes(x) => x == &1,
        }
    }
}

// Main function
pub fn day_16() {
    let input_file = "./input/day_16.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    find_sue(input, true)
}

fn find_sue(input: String, use_exact_match: bool) -> usize {
    input
        .lines()
        .enumerate()
        .find_map(|(index, line)| {
            let strings: Vec<String> = line.split_whitespace().skip(2).map(|s| s.to_string()).collect();
            strings
                .chunks(2)
                .filter_map(|chunk| {
                    let name = chunk.get(0)?;
                    let val = chunk.get(1)?;
                    let value: usize = val
                        .replace(",", "")
                        .parse()
                        .expect("Expecting a number");
                    Some(Property::from(name.to_string(), value))
                })
                .all(|prop| prop.matches_expected(use_exact_match))
                .then(|| index + 1)
        })
        .expect("No sue found")
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    find_sue(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("./input/day_16.txt"), 103);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input/day_16.txt"), 405);
    }
}
