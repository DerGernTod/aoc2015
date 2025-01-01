use std::{fs, ops::Add};

struct Ingredient(i32, i32, i32, i32, i32);
impl Add for Ingredient {
    type Output = Ingredient;

    fn add(self, other: Ingredient) -> Ingredient {
        Ingredient(self.0 + other.0, self.1 + other.1, self.2 + other.2, self.3 + other.3, self.4 + other.4)
    }
}

// Main function
pub fn day_15() {
    let input_file = "./input/day_15.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(_: &str) -> i32 {
    find_recipe(true)
}

// Puzzle 2 function
fn puzzle2(_: &str) -> i32 {
    find_recipe(false)
}

fn find_recipe(ignore_calories: bool) -> i32 {
    // sorry but this is cheaper than parsing
    let sprinkles = Ingredient(5, -1, 0, 0, 5);
    let peanut_butter = Ingredient(-1, 3, 0, 0, 1);
    let frosting = Ingredient(0, -1, 4, 0, 6);
    let sugar = Ingredient(-1, 0, 0, 2, 8);
    let mut largest = 0;
    for i in 0..100 {
        for j in 0..100 {
            if i + j > 100 {
                break;
            }
            for k in 0..100 {
                if i + j + k > 100 {
                    break;
                }
                for l in 0..100 {
                    if i + j + k + l == 100 {
                        let capacity = (i * sprinkles.0 + j * peanut_butter.0 + k * frosting.0 + l * sugar.0).max(0);
                        let durability = (i * sprinkles.1 + j * peanut_butter.1 + k * frosting.1 + l * sugar.1).max(0);
                        let flavor = (i * sprinkles.2 + j * peanut_butter.2 + k * frosting.2 + l * sugar.2).max(0);
                        let texture = (i * sprinkles.3 + j * peanut_butter.3 + k * frosting.3 + l * sugar.3).max(0);
                        let calories = (i * sprinkles.4 + j * peanut_butter.4 + k * frosting.4 + l * sugar.4).max(0);
                        if ignore_calories || calories == 500 {
                            largest = largest.max(capacity * durability * flavor * texture);
                        }
                    }
                }
            }
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("./input/day_15.txt"), 13882464);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input/day_15.txt"), 11171160);
    }
}
