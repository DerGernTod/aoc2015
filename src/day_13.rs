use std::{fs, str::FromStr, collections::{HashMap, BinaryHeap}, rc::Rc};

use self::{rating::Rating, node::Node};

mod rating;
mod node;

#[allow(dead_code)]
pub fn day_13() {
    let mut ratings = parse_into_ratings("./input/day_13.txt");
    let happiness = find_longest_path(&ratings);
    println!("Most happiness achieved with {happiness:?}");
    add_self(&mut ratings);
    let happiness = find_longest_path(&ratings);
    println!("Most happiness including me achieved with {happiness:?}");
}

fn parse_into_ratings(path: &str) -> HashMap<String, Vec<Rating>> {
    let input = fs::read_to_string(path).unwrap();
    input
        .lines()
        .flat_map(Rating::from_str)
        .fold(HashMap::new(), |mut ratings, rating| {
            let person_ratings = ratings.entry(rating.name.to_string()).or_default();
            person_ratings.push(rating);
            ratings
        })
}

fn add_self(ratings: &mut HashMap<String, Vec<Rating>>) {
    let all_names: Vec<String> = ratings.keys().map(|key| key.to_string()).collect();
    for name in all_names {
        let neighbors = ratings.entry(String::from("Me")).or_default();
        neighbors.push(Rating {happiness: 0, name: String::from("Me"), neighbor: name.to_string()});
        let name_copy = name.to_string();
        ratings
            .entry(name)
            .and_modify(|neighbors| neighbors.push(Rating {happiness: 0, name: name_copy, neighbor: String::from("Me")}));
    }
}

fn find_longest_path(ratings: &HashMap<String, Vec<Rating>>) -> i32 {
    let mut steps: HashMap<Rc<Vec<&String>>, i32> = HashMap::new();
    
    let total_neighbors = ratings.keys().count();
    let mut remaining: BinaryHeap<Node> = BinaryHeap::new();
    let start_step = vec![ratings.keys().next().unwrap()];
    remaining.push(Node { happiness: 0, current_path: Rc::new(start_step)});

    while let Some(Node { current_path, happiness }) = remaining.pop() {
        if happiness < *steps.get(&current_path).unwrap_or(&i32::MIN) {
            continue;
        }
        let neighbors = ratings
            .get(&current_path.last().unwrap().to_string())
            .unwrap()
            .iter()
            .filter(|rating| (current_path.len() == total_neighbors && rating.neighbor == *current_path[0])
                || !current_path.contains(&&rating.neighbor));
        for path in neighbors {
            let mut new_path = current_path.to_vec();
            new_path.push(&path.neighbor);
            let new_happiness = happiness + calc_combined_rating(path, ratings);
            
            if new_happiness > *steps.get(&new_path).unwrap_or(&i32::MIN) {
                let path_rc = Rc::new(new_path);
                remaining.push(Node {
                    happiness: new_happiness,
                    current_path: Rc::clone(&path_rc)
                });
                steps.insert(path_rc, new_happiness);
            }
        }
    }
    *steps.values().max().unwrap()
}

fn calc_combined_rating(rating: &Rating, ratings: &HashMap<String, Vec<Rating>>) -> i32 {
    rating.happiness
        + ratings.get(&rating.neighbor).unwrap().iter()
            .find(|other_rating| other_rating.neighbor == rating.name)
            .map(|rating| rating.happiness)
            .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{parse_into_ratings, find_longest_path};

    #[test]
    fn test_part_1() {
        let ratings = parse_into_ratings("./input/day_13.test.txt");
        assert_eq!(find_longest_path(&ratings), 330);
    }
}