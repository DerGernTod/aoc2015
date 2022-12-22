use std::collections::HashSet;

#[allow(dead_code)]
pub fn day_11() {
    println!("Next pw after cqjxjnds is {}", calc_next_password("cqjxjnds"));
    println!("The next one is {}", calc_next_password(&calc_next_password("cqjxjnds")));
}

fn calc_next_password(str: &str) -> String {
    let mut cur_pw = increment_pw(str);
    while !is_valid(&cur_pw) {
        cur_pw = increment_pw(&cur_pw);
    }
    cur_pw
}

fn increment_pw(str: &str) -> String {
    let mut chars_rev = vec![];
    let mut increase_next = true;
    for ch in str.chars().rev() {
        if increase_next {
            let next = (ch as u8 + 1) as char;
            increase_next = false;
            chars_rev.push(match next {
                '{' => {
                    increase_next = true;
                    'a'
                },
                'i' => 'j',
                'o' => 'p',
                'l' => 'm',
                _ => next
            });
        } else {
            chars_rev.push(ch);
        }
    }
    chars_rev.iter().rev().collect()
}

fn is_valid(str: &str) -> bool {
    has_no_invalids(str) && has_pairs(str) && has_straight(str)
}

fn has_pairs(str: &str) -> bool {
    let mut chars = str.chars();
    let mut prev = chars.next().unwrap();
    let mut known_pairs = HashSet::new();
    for ch in chars {
        if ch == prev {
            known_pairs.insert(ch);
        }
        prev = ch;
    }
    known_pairs.len() >= 2
}

fn has_no_invalids(str: &str) -> bool {
    !(str.contains('i') || str.contains('o') || str.contains('l'))
}

fn has_straight(str: &str) -> bool {
    let mut chars = str.chars();
    let mut prev = chars.next().unwrap();
    let mut consec_count = 0;
    for ch in chars {
        let follow_ch = (prev as u8 + 1) as char;
        if ch == follow_ch && follow_ch <= 'z' {
            consec_count += 1;
            if consec_count == 2 {
                return true;
            }
        } else {
            consec_count = 0;
        }
        prev = ch;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::{has_straight, has_pairs, calc_next_password};
    #[test]
    fn test_has_pairs() {
        println!("{}", ('z' as u8 + 1) as char);
        assert!(has_pairs("aabb"));
        assert!(!has_pairs("aaa"));
        assert!(!has_pairs("baa"));
        assert!(!has_pairs("abba"));
        assert!(has_pairs("aaasldfccbb"));
        assert!(has_pairs("abcdffaa"))
    }
    #[test]
    fn test_has_straight() {
        assert!(has_straight("abc"));
        assert!(!has_straight("adbc"));
        assert!(!has_straight("yza"));
        assert!(!has_straight("abddcd"));
        assert!(has_straight("abcdffaa"))
    }
    #[test]
    fn test_part_1() {
        assert_eq!(calc_next_password("abcdefgh"), "abcdffaa");
        assert_eq!(calc_next_password("ghjaaaaa"), "ghjaabcc");
    }
}