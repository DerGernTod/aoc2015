use std::fs;

#[allow(dead_code)]
pub fn day_01() {
    let (final_floor, cellar_index) = walk_floors("./input/day_01.txt");
    println!("Santa reached floor {}", final_floor);
    println!("Reached floor at {}", cellar_index + 1);
}

fn walk_floors(path: &str) -> (i32, usize) {
    let input = fs::read_to_string(path).unwrap();
    let (up, down, cellar_index) = input.chars().enumerate().fold((0, 0, usize::MAX), |(mut up, mut down, mut cellar_index), (index, ch)| {
        match ch {
            ')' => down += 1,
            '(' => up += 1,
            _ => panic!("Unknown character: {ch}")
        };
        if cellar_index == usize::MAX && down > up {
            cellar_index = index;
        }
        (up, down, cellar_index)
    });
    (up - down, cellar_index)
}
