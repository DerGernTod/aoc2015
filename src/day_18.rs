use std::{collections::HashMap, fs};

use aoc2015::{lib::vec2d::DIRECTIONS_8, Vec2D};

#[cfg(test)]
const MAP_SIZE: isize = 6;
#[cfg(test)]
const STEPS: usize = 5;

#[cfg(not(test))]
const MAP_SIZE: isize = 100;
#[cfg(not(test))]
const STEPS: usize = 100;

// Main function
pub fn day_18() {
    let input_file = "./input/day_18.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    let mut map = read_map(&input);
    for _ in 0..100 {
        map = next_state(&map);
    }
    map
        .into_iter()
        .filter(|(_, state)| *state)
        .count()
}

fn next_state(map: &HashMap<Vec2D, bool>) -> HashMap<Vec2D, bool> {
    map
        .into_iter()
        .map(|(pos, state)| {
            (*pos, evaluate_new_state(map, pos, state))
        })
        .collect()
}

fn evaluate_new_state(map: &HashMap<Vec2D, bool>, pos: &Vec2D, state: &bool) -> bool {
    let neighbors_on = count_active_neighbors(map, pos);
    match (state, neighbors_on) {
        (true, 2..=3) => true,
        (false, 3) => true,
        _ => false,
    }
}

fn count_active_neighbors(map: &HashMap<Vec2D, bool>, pos: &Vec2D) -> usize {
    DIRECTIONS_8
        .into_iter()
        .map(|dir| *pos + dir)
        .filter(|pos| *map.get(pos).unwrap_or(&false))
        .count()
}

fn read_map(input: &str) -> HashMap<Vec2D, bool> {
    input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| line
            .chars()
            .enumerate()
            .map(move |(x, char)| (
                Vec2D::new(x as isize, y as isize),
                if char == '#' { true } else { false }
            ))
        )
        .collect()
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    let mut map = read_map(&input);

    for _ in 0..STEPS {
        assign_corners(&mut map);
        map = next_state_locked(&map);
    }
    assign_corners(&mut map);
    map
        .into_iter()
        .filter(|(_, state)| *state)
        .count()
}

fn assign_corners(map: &mut HashMap<Vec2D, bool>) {
    let corners = vec![
        Vec2D::new(0, 0),
        Vec2D::new(MAP_SIZE - 1, 0),
        Vec2D::new(0, MAP_SIZE - 1),
        Vec2D::new(MAP_SIZE - 1, MAP_SIZE - 1),
    ];
    map.extend(corners
        .into_iter()
        .map(|corner| {
            (corner, true)
    }));
}

fn is_not_corner(pos: &Vec2D) -> bool {
    !((pos.x() == 0 && (pos.y() == 0 || pos.y() == MAP_SIZE - 1))
    || (pos.x() == MAP_SIZE - 1 && (pos.y() == 0 || pos.y() == MAP_SIZE - 1)))
    
}

fn next_state_locked(map: &HashMap<Vec2D, bool>) -> HashMap<Vec2D, bool> {
    map
        .into_iter()
        .filter(|(pos, _)| is_not_corner(pos))
        .map(|(pos, state)| {
            (*pos, evaluate_new_state(map, pos, state))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("./input_test/day_18.txt"), 4);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input_test/day_18.txt"), 17);
    }
}
