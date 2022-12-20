use std::{collections::HashSet, fs};

#[allow(dead_code)]
pub fn day_03() {
    let visited = parse_into_visited_locations("./input/day_03.txt");
    println!("Santa visited {} houses", visited.len());
    let visited = parse_into_visited_locations_with_robo_santa("./input/day_03.txt");
    println!("Santa visited {} houses with robo santa", visited.len());
}

fn parse_into_visited_locations_with_robo_santa(path: &str) -> HashSet<(i32, i32)> {
    let input = fs::read_to_string(path).unwrap();
    input
        .chars()
        .enumerate()
        .fold(((0, 0), (0, 0), HashSet::new()), |(mut robo, mut real, mut visited), (index, ch)| {
            let to_update = match ch {
                '^' => (0, 1),
                '<' => (-1, 0),
                '>' => (1, 0),
                'v' => (0, -1),
                _ => panic!("Unexpected char: {ch}")
            };
            if index % 2 == 0{
                robo = (robo.0 + to_update.0, robo.1 + to_update.1);
                visited.insert(robo);
            } else {
                real = (real.0 + to_update.0, real.1 + to_update.1);
                visited.insert(real);
            }
            (robo, real, visited)
        }).2
}

fn parse_into_visited_locations(path: &str) -> HashSet<(i32, i32)> {
    let input = fs::read_to_string(path).unwrap();
    input
        .chars()
        .fold(((0, 0), HashSet::new()), |((x, y), mut visited), ch| {
            let new_pos = match ch {
                '^' => (x, y + 1),
                '<' => (x - 1, y),
                '>' => (x + 1, y),
                'v' => (x, y - 1),
                _ => panic!("Unexpected char: {ch}")
            };
            visited.insert(new_pos);
            (new_pos, visited)
        }).1
}

#[cfg(test)]
mod tests {
    use crate::day_03::parse_into_visited_locations_with_robo_santa;

    use super::parse_into_visited_locations;

    #[test]
    fn test_part_1() {
        let visited = parse_into_visited_locations("./input/day_03.test.txt");
        assert_eq!(visited.len(), 4);
    }

    #[test]
    fn test_part_2() {
        let visited = parse_into_visited_locations_with_robo_santa("./input/day_03.test.txt");
        assert_eq!(visited.len(), 3);
    }
}