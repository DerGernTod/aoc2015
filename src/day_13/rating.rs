use std::{str::FromStr, num::ParseIntError};

#[derive(PartialEq, Eq, Debug)]
pub struct Rating {
    pub name: String,
    pub neighbor: String,
    pub happiness: i32
}

impl FromStr for Rating {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let simplified = s
            .replace("gain ", "")
            .replace("lose ", "-");
        let mut spl = simplified.split(' ');
        let name = spl.next().unwrap().to_string();
        let happiness = spl.nth(1).unwrap().parse::<i32>()?;
        let neighbor = spl.nth(6)
            .unwrap()
            .strip_suffix('.')
            .unwrap()
            .to_string();
        Ok(Self {
            happiness,
            name,
            neighbor
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Rating;
    #[test]
    fn test_from_str() {
        let rating = Rating::from_str("Alice would gain 54 happiness units by sitting next to Bob.").unwrap();
        assert_eq!(rating, Rating {
            name: String::from("Alice"),
            neighbor: String::from("Bob"),
            happiness: 54
        });
        let rating = Rating::from_str("Bob would lose 54 happiness units by sitting next to Alice.").unwrap();
        assert_eq!(rating, Rating {
            name: String::from("Bob"),
            neighbor: String::from("Alice"),
            happiness: -54
        });
    }
}