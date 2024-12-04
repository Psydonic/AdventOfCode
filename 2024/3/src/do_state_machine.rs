pub struct DoStateMachine {
    pub state: DoState,
    pub current_string: String,
    pub matched_strings: Vec<String>,
}

pub enum DoState {
    Start,
    D,
    O,
    OpenParenthesis
}

impl DoStateMachine {
    pub fn new() -> DoStateMachine {
        DoStateMachine { state: DoState::Start, current_string: String::new(), matched_strings: Vec::new() }
    }

    pub fn transform(&mut self, c: char) {
        match self.state {
            DoState::Start => {
                if c == 'd' {
                    self.state = DoState::D;
                    self.current_string.push(c);
                }
            },
            DoState::D => {
                if c == 'o' {
                    self.state = DoState::O;
                    self.current_string.push(c);
                } else {
                    self.state = DoState::Start;
                    self.current_string.clear();
                }
            },
            DoState::O => {
                if c == '(' {
                    self.state = DoState::OpenParenthesis;
                    self.current_string.push(c);
                } else {
                    self.state = DoState::Start;
                    self.current_string.clear();
                }
            },
            DoState::OpenParenthesis => {
                if c == ')' {
                    self.state = DoState::Start;
                    self.current_string.push(c);
                    self.matched_strings.push(self.current_string.clone()); // Add the current string to the list of matched strings
                    self.current_string.clear(); // Clear the current string after adding it to the list
                } else {
                    self.state = DoState::Start;
                    self.current_string.clear();
                }
            }
        }
    }
}

