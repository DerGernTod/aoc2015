use std::fs;

struct Gift(usize, usize, usize);

impl Gift {
    fn calc_wrapping_area(&self) -> usize {
        let Gift(w, h, b) = self;
        let top = w * b;
        let front = b * h;
        let side = w * h;
        2 * front + 2 * top + 2 * side + top.min(front).min(side)
    }
    fn calc_ribbon_length(&self) -> usize {
        let Gift(w, h, b) = self;
        let small_a = w.min(h).min(b);
        let small_b = if small_a == w { h.min(b) } else if small_a == b { h.min(w) } else { w.min(b) };
        2 * small_a + 2 * small_b + w * h * b
    }
}

#[allow(dead_code)]
pub fn day_02() {
    let gifts = parse_into_gifts("./input/day_02.txt");
    println!("The elves need {} sqf wrapping paper, with {} feet of ribbon.", calc_wrapping_area(&gifts), calc_ribbon_length(&gifts));
}

fn calc_wrapping_area(gifts: &[Gift]) -> usize {
    gifts
        .iter()
        .map(Gift::calc_wrapping_area)
        .sum()
}

fn calc_ribbon_length(gifts: &[Gift]) -> usize {
    gifts
        .iter()
        .map(Gift::calc_ribbon_length)
        .sum()
}

fn parse_into_gifts(path: &str) -> Vec<Gift> {
    let input = fs::read_to_string(path).unwrap();
    input
        .lines()
        .map(|line| {
            let mut line_iter = line
                .split('x')
                .map(|size| size.parse::<usize>().unwrap());
            Gift(line_iter.next().unwrap(), line_iter.next().unwrap(), line_iter.next().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day_02::{parse_into_gifts, calc_wrapping_area, calc_ribbon_length};

    #[test]
    fn test_part_1() {
        let gifts = parse_into_gifts("./input/day_02.test.txt");
        assert_eq!(101, calc_wrapping_area(&gifts));
    }

    #[test]
    fn test_part_2() {
        let gifts = parse_into_gifts("./input/day_02.test.txt");
        assert_eq!(48, calc_ribbon_length(&gifts));
    }
}