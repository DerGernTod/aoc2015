use std::{fs, thread::yield_now};

// Main function
pub fn day_17() {
    let input_file = "./input/day_17.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    let mut containers: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    containers.sort();
    println!("{:?}", containers);
    find_combinations(&containers, 0, 0, 0)
}
// 188319355 too high
// 1761 too low
fn find_combinations(containers: &Vec<usize>, offset: usize, sum: usize, num_iterations: usize) -> usize {
    let mut combinations = 0;
    for i in offset..containers.len() {
        let cur_sum = sum + containers[i];
        if cur_sum == 150 {
            println!("found result with {} iterations: {:?}", num_iterations, containers[i]);
            combinations += 1;
        } else if cur_sum > 150 {
            continue;
        } else if cur_sum + containers[i] > 150 {
            continue;
        } else {
            combinations += find_combinations(containers, i + 1, cur_sum, num_iterations + 1)
        }
    }
    combinations
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("./input_test/day_17.txt"), 4372);
    }

    #[test]
    fn test_puzzle2() {
        // least iterations is 3, number of combinations is 4
        assert_eq!(puzzle2("./input_test/day_17.txt"), 4);
    }
}
