use std::fs;

#[allow(dead_code)]
pub fn day_04() {
    let test = fs::read_to_string("./input/day_04.txt").unwrap();
    let lowest = find_lowest_num_leading_zeroes(&test, 5);
    println!("Advent coin mined at {}!", lowest);
    let lowest = find_lowest_num_leading_zeroes(&test, 6);
    println!("Another Advent coin mined at {}!", lowest);
}

fn find_lowest_num_leading_zeroes(secret: &String, num_zeroes: usize) -> usize {
    let mut res = None;
    let mut counter = 0;
    while res.is_none() {
        let digest = md5::compute(format!("{}{}", secret, counter));
        if format!("{:?}", digest).starts_with(&"0".repeat(num_zeroes)) {
            res = Some(counter);
        }
        counter += 1;
    }
    res.unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day_04::find_lowest_num_leading_zeroes;
    #[test]
    fn test_part_1() {
        let test = fs::read_to_string("./input/day_04.test.txt").unwrap();
        let lowest = find_lowest_num_leading_zeroes(&test, 5);
        assert_eq!(lowest, 609043);
    }
}