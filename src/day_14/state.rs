#[derive(Debug, PartialEq, Eq)]
pub struct State {
    pub remaining_seconds: usize,
    pub speed: usize,
    pub next_state_type: StateType
}

#[derive(Debug, PartialEq, Eq)]
pub enum StateType {
    Wait,
    Run
}

impl State {
    pub fn exec_next_second(&mut self) -> Result<usize, usize> {
        self.remaining_seconds -= 1;
        if self.remaining_seconds == 0 {
            Err(self.speed)
        } else {
            Ok(self.speed)
        }
    }
}

