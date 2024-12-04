pub struct MulStateMachine {
    pub state: MulState,
    pub current_string: String,
    pub matched_strings: Vec<String>,
}

pub enum MulState {
    Start,
    M,
    U,
    L,
    OpenParenthesis,
    FirstNumber,
    Comma,
    SecondNumber,
}

impl MulStateMachine {
    pub fn new() -> MulStateMachine {
        MulStateMachine { state: MulState::Start, current_string: String::new(), matched_strings: Vec::new() }
    }

    pub fn transform(&mut self, c: char) {
        match self.state {
            MulState::Start => {
                if c == 'm' {
                    self.state = MulState::M;
                    self.current_string.push(c);
                }
            },
            MulState::M => {
                if c == 'u' {
                    self.state = MulState::U;
                    self.current_string.push(c);
                } else {
                    self.state = MulState::Start;
                    self.current_string.clear();
                }
            },
            MulState::U => {
                if c == 'l' {
                    self.state = MulState::L;
                    self.current_string.push(c);
                } else {
                    self.state = MulState::Start;
                    self.current_string.clear();
                }
            },
            MulState::L => {
                if c == '(' {
                    self.state = MulState::OpenParenthesis;
                    self.current_string.push(c);
                } else {
                    self.state = MulState::Start;
                    self.current_string.clear();
                }
            },
            MulState::OpenParenthesis => {
                if c.is_digit(10) {
                    self.state = MulState::FirstNumber;
                    self.current_string.push(c);
                } else {
                    self.state = MulState::Start;
                    self.current_string.clear();
                }
            },
            MulState::FirstNumber => {
                if c == ',' {
                    self.state = MulState::Comma;
                    self.current_string.push(c);
                } else if c.is_digit(10) {
                    self.current_string.push(c);
                } else {
                    self.state = MulState::Start;
                    self.current_string.clear();
                }
            },
            MulState::Comma => {
                if c.is_digit(10) {
                    self.state = MulState::SecondNumber;
                    self.current_string.push(c);
                } else {
                    self.state = MulState::Start;
                    self.current_string.clear();
                }
            },
            MulState::SecondNumber => {
                if c == ')' {
                    self.state = MulState::Start;
                    self.current_string.push(c);
                    self.matched_strings.push(self.current_string.clone()); // Add the current string to the list of matched strings
                    self.current_string.clear(); // Clear the current string after adding it to the list
                } else if c.is_digit(10) {
                    self.current_string.push(c);
                } else {
                    self.state = MulState::Start;
                    self.current_string.clear();
                }
            }
        }
    }
}
