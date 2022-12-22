use std::fs;

#[allow(dead_code)]
pub fn day_08() {
    println!("String combination count: {}", count_inner_outer_string_length("./input/day_08.txt"));
    println!("Encoded string combination count: {}", count_encoded_inner_outer_string_length("./input/day_08.txt"));
}

fn count_inner_outer_string_length(path: &str) -> i32 {
    let input = fs::read_to_string(path).unwrap();
    let (inner, outer) = input
        .lines()
        .map(|line| (line.chars().count(), {
            let line_len = line.len();
            let no_quotes_line = &line[1..line_len - 1];
            let no_quotes_line = no_quotes_line
                .replace("\\\\", ".")
                .replace("\\\"", ".");
            let utf8_count = no_quotes_line.matches("\\x").count();
            no_quotes_line.len() - utf8_count * 3
        }))
        .fold((0, 0), |(inner, outer), (cur_inner, cur_outer)| (inner + cur_inner, outer + cur_outer));
    inner as i32 - outer as i32
}
fn count_encoded_inner_outer_string_length(path: &str) -> i32 {
    let input = fs::read_to_string(path).unwrap();
    let (inner, outer) = input
        .lines()
        .map(|line| (line.chars().count(), {
            let mut count = 2;
            for ch in line.chars() {
                if matches!(ch, '"' | '\\') {
                    count += 2;
                } else {
                    count += 1;
                }
            }
            count
        }))
        .fold((0, 0), |(inner, outer), (cur_inner, cur_outer)| (inner + cur_inner, outer + cur_outer));
    outer as i32 - inner as i32
}

#[cfg(test)]
mod tests {
    use crate::day_08::{count_inner_outer_string_length, count_encoded_inner_outer_string_length};

    #[test]
    fn test_part_1() {
        assert_eq!(count_inner_outer_string_length("./input/day_08.test.txt"), 12);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(count_encoded_inner_outer_string_length("./input/day_08.test.txt"), 19);
    }
}