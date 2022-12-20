use std::fs;

pub fn day_05() {
    println!("Santa has {} nice strings", count_nice_strings("./input/day_05.txt"));
    println!("Santa has {} nicer strings", count_nicer_strings("./input/day_05.txt"));
}

fn is_nicer(str: &&str) -> bool {
    let chars: Vec<char> = str.chars().collect();
    let repeats_with_letter_between = chars.windows(3).any(|window| window[0] == window[2]);
    let has_pair_twice = chars
        .windows(2)
        .any(|window| str.matches(&format!("{}{}", window[0], window[1])).count() >= 2);
    repeats_with_letter_between && has_pair_twice
}

fn is_nice(str: &&str) -> bool {
    if str.contains("ab") || str.contains("cd") || str.contains("pq") || str.contains("xy") {
        false
    } else {
        let chars: Vec<char> = str.chars().collect();
        let has_triplet = chars.windows(2).any(|window| window[0] == window[1]);
        let has_3_vowels = chars.iter().filter(|ch| matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u')).count() >= 3;
        has_triplet && has_3_vowels
    }
}

fn count_nice_strings(path: &str) -> usize {
    let input = fs::read_to_string(path).unwrap();
    input
        .lines()
        .filter(is_nice)
        .count()
}

fn count_nicer_strings(path: &str) -> usize {
    let input = fs::read_to_string(path).unwrap();
    input
        .lines()
        .filter(is_nicer)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day_05::count_nicer_strings;

    use super::count_nice_strings;

    #[test]
    fn test_part_1() {
        assert_eq!(count_nice_strings("./input/day_05.test.txt"), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(count_nicer_strings("./input/day_05.test.2.txt"), 2);
    }
}