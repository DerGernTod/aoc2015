use std::{fs, str::FromStr};

use self::reindeer::Reindeer;

mod reindeer;
mod state;

#[allow(dead_code)]
pub fn day_14() {
    let mut reindeers = read_into_reindeers("./input/day_14.txt");
    println!("Best reindeer distance: {} km!", lead_distance_after_x_seconds(2503, &mut reindeers));
    let mut reindeers = read_into_reindeers("./input/day_14.txt");
    println!("Highest reindeer score: {}!", lead_score_after_x_seconds(2503, &mut reindeers));
}


fn lead_distance_after_x_seconds(seconds: usize, reindeers: &mut [Reindeer]) -> usize {
    for _ in 0..seconds {
        reindeers.iter_mut().for_each(|reindeer| reindeer.exec_next_second());
    }
    calc_best_reindeer(reindeers)
}

fn lead_score_after_x_seconds(seconds: usize, reindeers: &mut [Reindeer]) -> usize {
    for _ in 0..seconds {
        reindeers.iter_mut().for_each(|reindeer| reindeer.exec_next_second());
        let dist = reindeers.iter().max_by_key(|reindeer| reindeer.distance).map(|reindeer| reindeer.distance).unwrap();
        reindeers.iter_mut().filter(|reindeer| reindeer.distance == dist).for_each(|reindeer| reindeer.score += 1);
    }
    reindeers.iter().max_by_key(|reindeer| reindeer.score).unwrap().score
}

fn calc_best_reindeer(reindeers: &[Reindeer]) -> usize {
    reindeers.iter().max_by_key(|reindeer| reindeer.distance).unwrap().distance
}

fn read_into_reindeers(path: &str) -> Vec<Reindeer> {
    let input = fs::read_to_string(path).unwrap();
    input.lines().flat_map(Reindeer::from_str).collect()
}

#[cfg(test)]
mod tests {
    use super::{read_into_reindeers, lead_score_after_x_seconds};

    #[test]
    fn test_part_1() {
        let mut reindeers = read_into_reindeers("./input/day_14.test.txt");
        assert_eq!(reindeers.len(), 2);
        for _ in 0..1000 {
            reindeers.iter_mut().for_each(|reindeer| reindeer.exec_next_second());
        }
        assert_eq!(reindeers[0].distance, 1120);
        assert_eq!(reindeers[1].distance, 1056);
    }
    #[test]
    fn test_part_2() {
        let mut reindeers = read_into_reindeers("./input/day_14.test.txt");
        let score = lead_score_after_x_seconds(1000, &mut reindeers);
        assert_eq!(score, 689);
    }
}