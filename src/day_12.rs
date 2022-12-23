mod json;

use std::{fs, num::ParseIntError, str::Chars};

#[derive(PartialEq, Eq)]
enum Reset {
    Both,
    String,
    None
}

#[allow(dead_code)]
pub fn day_12() {
    println!("All numbers summed up: {}", read_into_numbers("./input/day_12.txt").unwrap());
    println!("All numbers without red properties: {}", read_into_non_red_numbers("./input/day_12.txt").unwrap());
}

fn read_into_numbers(path: &str) -> Result<i32, ParseIntError> {
    let input = fs::read_to_string(path).unwrap();
    sum_json_numbers(&input)
}

fn read_into_non_red_numbers(path: &str) -> Result<i32, ParseIntError> {
    let input = fs::read_to_string(path).unwrap();
    get_num_sum_in_object(&mut input.chars())
}

fn sum_json_numbers(str: &str) -> Result<i32, ParseIntError> {
    let mut cur_num = 0;
    let mut is_string = false;
    let mut cur_num_chars: Vec<char> = vec![];
    for ch in str.chars() {
        let finish_num = match ch {
            '-' if !is_string && cur_num_chars.is_empty() => { cur_num_chars.push(ch); false },
            '0'..='9' if !is_string => { cur_num_chars.push(ch); false },
            '"' => { is_string = !is_string; true },
            _ => true
        };
        if finish_num && !cur_num_chars.is_empty() {
            cur_num += cur_num_chars.iter().collect::<String>().parse::<i32>()?;
            cur_num_chars.clear();
        }
    }
    Ok(cur_num)
}

fn get_num_sum_in_array(char_iter: &mut Chars) -> Result<i32, ParseIntError> {
    let mut cur_sum = 0;
    let mut is_string = false;
    let mut cur_num_chars: Vec<char> = vec![];
    while let Some(ch) = char_iter.next() {
        let reset = match ch {
            '-' if !is_string && cur_num_chars.is_empty() => { cur_num_chars.push(ch); Reset::None },
            '0'..='9' if !is_string => { cur_num_chars.push(ch); Reset::None },
            '"' => { is_string = !is_string; Reset::Both },
            '{' if !is_string => {
                cur_sum += get_num_sum_in_object(char_iter)?;
                Reset::None
            },
            '[' if !is_string => {
                cur_sum += get_num_sum_in_array(char_iter)?;
                Reset::None
            },
            ']' if !is_string => {
                Reset::Both
            }
            _ => Reset::Both
        };
        if reset == Reset::Both && !cur_num_chars.is_empty() {
            cur_sum += cur_num_chars.iter().collect::<String>().parse::<i32>()?;
            cur_num_chars.clear();
        }
        if reset == Reset::Both && !is_string && ch == ']' {
            break;
        }
    }
    Ok(cur_sum)
}

fn get_num_sum_in_object(char_iter: &mut Chars) -> Result<i32, ParseIntError> {
    let mut cur_sum = 0;
    let mut is_string = false;
    let mut cur_string = vec![];
    let mut cur_num_chars: Vec<char> = vec![];
    let mut has_red = false;
    while let Some(ch) = char_iter.next() {
        
        let reset = match ch {
            '-' if !is_string && cur_num_chars.is_empty() => { cur_num_chars.push(ch); Reset::None },
            '0'..='9' if !is_string => { cur_num_chars.push(ch); Reset::None },
            '"' => { is_string = !is_string; Reset::String },
            'a'..='z' => {
                if is_string {
                    cur_string.push(ch);
                }
                Reset::None
            },
            '{' if !is_string => {
                cur_sum += get_num_sum_in_object(char_iter)?;
                Reset::None
            },
            '[' if !is_string => {
                cur_sum += get_num_sum_in_array(char_iter)?;
                Reset::None
            }
            '}' if !is_string => {
                Reset::Both
            }
            _ => Reset::Both
        };
        if matches!(reset, Reset::Both | Reset::String) && !cur_string.is_empty() {
            if cur_string.iter().collect::<String>() == "red" {
                has_red = true;
            }
            cur_string.clear();
        } 
        if reset == Reset::Both && !cur_num_chars.is_empty() {
            cur_sum += cur_num_chars.iter().collect::<String>().parse::<i32>()?;
            cur_num_chars.clear();
        }
        if reset == Reset::Both && !is_string && ch == '}' {
            break;
        }
    }
    if has_red {
        Ok(0)
    } else {
        Ok(cur_sum)
    }
}

#[cfg(test)]
mod tests {
    use crate::day_12::{get_num_sum_in_array, get_num_sum_in_object};

    use super::sum_json_numbers;

    #[test]
    fn test_get_num_sum_in_array() {
        assert_eq!(get_num_sum_in_array(&mut "[1,2,3]".chars()), Ok(6));
        assert_eq!(get_num_sum_in_array(&mut "[1,{\"c\":\"red\",\"b\":2},3]".chars()), Ok(4));
        assert_eq!(get_num_sum_in_array(&mut "[1,\"red\",5]".chars()), Ok(6));
    }

    #[test]
    fn test_get_num_sum_in_object() {
        assert_eq!(get_num_sum_in_object(&mut "{\"d\":\"red\",\"e\":1}".chars()), Ok(0));
        assert_eq!(get_num_sum_in_object(&mut "{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}".chars()), Ok(0));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(sum_json_numbers("{\"a\":2,\"b\":4}"), Ok(6));
        assert_eq!(sum_json_numbers("{\"a\":{\"b\":4},\"c\":-1}"), Ok(3));
        assert_eq!(sum_json_numbers("[-1,{\"a\":1}]"), Ok(0));
    }
}