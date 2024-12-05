pub struct XmasStateMachine {
    pub state: XmasState,
}

pub enum XmasState {
    Start,
    X,
    XM,
    XMA,
    XMAS
}

impl XmasStateMachine {
    pub fn new() -> XmasStateMachine {
        XmasStateMachine { state: XmasState::Start }
    }

    pub fn transform(&mut self, c: char) {
        match self.state {
            XmasState::Start => {
                if c == 'x' || c == 'X' {
                    self.state = XmasState::X;
                }
            },
            XmasState::X => {
                if c == 'm' || c == 'M' {
                    self.state = XmasState::XM;
                } else {
                    self.state = XmasState::Start;
                }
            },
            XmasState::XM => {
                if c == 'a' || c == 'A' {
                    self.state = XmasState::XMA;
                } else {
                    self.state = XmasState::Start;
                }
            },
            XmasState::XMA => {
                if c == 's' || c == 'S' {
                    self.state = XmasState::XMAS;
                } else {
                    self.state = XmasState::Start;
                }
            },
            XmasState::XMAS => {
                if c == 'x' || c == 'X' {
                    self.state = XmasState::X;
                }
            }
        }
    }
}

