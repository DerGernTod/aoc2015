use std::{fs, str::FromStr, collections::HashMap};
use self::node::Node;

mod node;

enum Goal {
    Min,
    Max
}

#[allow(dead_code)]
pub fn day_09() {
    let nodes = parse_into_nodes("./input/day_09.txt");
    println!("Cheapest route is {} long", find_best_route(nodes, Goal::Min));
    let nodes = parse_into_nodes("./input/day_09.txt");
    println!("Most expensive route is {} long", find_best_route(nodes, Goal::Max));
}

fn parse_into_nodes(path: &str) -> Vec<Node> {
    let input = fs::read_to_string(path).unwrap();
    input.lines().flat_map(Node::from_str).collect()
}

fn build_all_routes_by_destination(nodes: Vec<Node>) -> HashMap<String, Vec<Node>> {
    nodes
        .into_iter()
        .fold(HashMap::new(), |mut map, node| {
            let node_name = node.name.to_string();
            let target_name = node.target.to_string();
            let reverse_node = Node::new_reversed(&node);
            let entry = map
                .entry(node_name)
                .or_default();
            entry.push(node);
            let entry_reverse = map
                .entry(target_name)
                .or_default();
            entry_reverse.push(reverse_node);
            map
        })
}

fn find_best_route(nodes: Vec<Node>, goal: Goal) -> usize {
    let unique_places: HashMap<String, Vec<Node>> = build_all_routes_by_destination(nodes);
    let mapped_nodes = unique_places
        .values()
        .map(|targets| match goal {
            Goal::Min => targets.iter().min_by_key(|node| node.cost).unwrap(),
            Goal::Max => targets.iter().max_by_key(|node| node.cost).unwrap(),
        })
        .map(|min_node| calc_costs_for_start_node(min_node, &unique_places, &goal));
    match goal {
        Goal::Min => mapped_nodes.min().unwrap(),
        Goal::Max => mapped_nodes.max().unwrap(),
    }
}

fn calc_costs_for_start_node(node: &Node, unique_places: &HashMap<String, Vec<Node>>, goal: &Goal) -> usize {
    let mut visited = vec![];
    let mut cur_target = Some(node);
    let mut cur_cost = 0;
    print!("{}", node.name);
    while let Some(target) = cur_target {
        print!("-> {}", target.target);
        cur_cost += target.cost;
        visited.push(&target.name);
        cur_target = find_best_unvisited_target(target, unique_places, &visited, goal);
    }
    println!(": {cur_cost}");
    cur_cost
}

fn find_best_unvisited_target<'a>(node: &Node, unique_places: &'a HashMap<String, Vec<Node>>, visited: &[&String], goal: &Goal) -> Option<&'a Node> {
    let filtered_targets = unique_places
        .get(&node.target)
        .unwrap()
        .iter()
        .filter(|target_node| !visited.contains(&&target_node.target));
    match goal {
        Goal::Min => filtered_targets.min_by_key(|node| node.cost),
        Goal::Max => filtered_targets.max_by_key(|node| node.cost),
    }
        
}


#[cfg(test)]
mod tests {
    use crate::day_09::{find_best_route, Goal};

    use super::parse_into_nodes;

    #[test]
    fn test_part_1() {
        let nodes = parse_into_nodes("./input/day_09.test.txt");
        assert_eq!(nodes.len(), 3);
        assert_eq!(find_best_route(nodes, Goal::Min), 605);
    }

    #[test]
    fn test_part_2() {
        let nodes = parse_into_nodes("./input/day_09.test.txt");
        assert_eq!(nodes.len(), 3);
        assert_eq!(find_best_route(nodes, Goal::Max), 982);
    }
}