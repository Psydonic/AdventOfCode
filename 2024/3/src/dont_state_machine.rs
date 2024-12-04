pub struct DontStateMachine {
    pub state: DontState,
    pub current_string: String,
    pub matched_strings: Vec<String>,
}

pub enum DontState {
    Start,
    D,
    O,
    N,
    Apostrophe,
    T,
    OpenParenthesis
}

impl DontStateMachine {
    pub fn new() -> DontStateMachine {
        DontStateMachine { state: DontState::Start, current_string: String::new(), matched_strings: Vec::new() }
    }

    pub fn transform(&mut self, c: char) {
        match self.state {
            DontState::Start => {
                if c == 'd' {
                    self.state = DontState::D;
                    self.current_string.push(c);
                }
            },
            DontState::D => {
                if c == 'o' {
                    self.state = DontState::O;
                    self.current_string.push(c);
                } else {
                    self.state = DontState::Start;
                    self.current_string.clear();
                }
            },
            DontState::O => {
                if c == 'n' {
                    self.state = DontState::N;
                    self.current_string.push(c);
                } else {
                    self.state = DontState::Start;
                    self.current_string.clear();
                }
            },
            DontState::N => {
                if c == '\'' {
                    self.state = DontState::Apostrophe;
                    self.current_string.push(c);
                } else {
                    self.state = DontState::Start;
                    self.current_string.clear();
                }
            },
            DontState::Apostrophe => {
                if c == 't' {
                    self.state = DontState::T;
                    self.current_string.push(c);
                } else {
                    self.state = DontState::Start;
                    self.current_string.clear();
                }
            },
            DontState::T => {
                if c == '(' {
                    self.state = DontState::OpenParenthesis;
                    self.current_string.push(c);
                } else {
                    self.state = DontState::Start;
                    self.current_string.clear();
                }
            },
            DontState::OpenParenthesis => {
                if c == ')' {
                    self.state = DontState::Start;
                    self.current_string.push(c);
                    self.matched_strings.push(self.current_string.clone()); // Add the current string to the list of matched strings
                    self.current_string.clear(); // Clear the current string after adding it to the list
                } else {
                    self.state = DontState::Start;
                    self.current_string.clear();
                }
            }
        }
    }
}

