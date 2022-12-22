#[allow(dead_code)]
pub fn day_10() {
    println!("String length after {} rounds: {}", 40, execute_rounds("1113122113", 40));
    println!("String length after {} rounds: {}", 50, execute_rounds("1113122113", 50));
}

fn execute_rounds(start: &str, num: usize) -> usize {
    let mut cur = String::from(start);
    for _ in 0..num {
        cur = evaluate_string(cur);
    }
    cur.len()
}

fn evaluate_string(str: String) -> String {
    let mut strings = vec![];
    let mut chars = str.chars();
    let mut cur_ch = chars.next().unwrap();
    let mut char_count = 1;
    for ch in chars {
        if ch == cur_ch {
            char_count += 1;
        } else {
            strings.push(format!("{}{}", char_count, cur_ch));
            cur_ch = ch;
            char_count = 1
        }
    }
    strings.push(format!("{}{}", char_count, cur_ch));
    strings.join("")
}

#[cfg(test)]
mod tests {
    use super::evaluate_string;

    #[test]
    fn test_part_1() {
        assert_eq!(evaluate_string(String::from("1")), "11");
        assert_eq!(evaluate_string(String::from("11")), "21");
        assert_eq!(evaluate_string(String::from("21")), "1211");
        assert_eq!(evaluate_string(String::from("1211")), "111221");
        assert_eq!(evaluate_string(String::from("111221")), "312211");
    }
}