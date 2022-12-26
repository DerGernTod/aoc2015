use std::{str::FromStr, num::ParseIntError};

use super::state::{State, StateType};

#[derive(Debug, PartialEq, Eq)]
pub struct Reindeer {
    name: String,
    speed: usize,
    stamina: usize,
    rest_time: usize,
    pub distance: usize,
    pub score: usize,
    state: State
}

impl Reindeer {
    pub fn exec_next_second(&mut self) {
        let round_result = self.state.exec_next_second();
        self.distance += match round_result { Err(x) | Ok(x) => x };
        if round_result.is_err() {
            self.state = match self.state.next_state_type {
                StateType::Wait => State {
                    remaining_seconds: self.rest_time,
                    speed: 0,
                    next_state_type: StateType::Run
                },
                StateType::Run => State {
                    remaining_seconds: self.stamina,
                    speed: self.speed,
                    next_state_type: StateType::Wait
                },
            }
        }
    } 
}

impl FromStr for Reindeer {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');
        let name = iter.next().unwrap().to_string();
        let speed = iter.nth(2).unwrap().parse::<usize>()?;
        let stamina = iter.nth(2).unwrap().parse::<usize>()?;
        let rest_time = iter.nth(6).unwrap().parse::<usize>()?;
        Ok(Reindeer {
            name,
            speed,
            stamina,
            rest_time,
            distance: 0,
            score: 0,
            state: State {
                remaining_seconds: stamina,
                next_state_type: StateType::Wait,
                speed
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::day_14::state::{State, StateType};

    use super::Reindeer;

    #[test]
    fn test_from_str() {
        assert_eq!(
            Reindeer::from_str("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.").unwrap(),
            Reindeer {
                name: "Comet".to_string(),
                speed: 14,
                stamina: 10,
                rest_time: 127,
                distance: 0,
                score: 0,
                state: State {
                    remaining_seconds: 10,
                    next_state_type: StateType::Wait,
                    speed: 14
                }
            }
        )
    }
}