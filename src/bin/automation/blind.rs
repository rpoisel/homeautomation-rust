use std::time::Instant;

pub trait State {
    fn next(&self) -> (Box<dyn State>, bool, bool);
}

pub struct StateIdle {
    up: bool,
    down: bool,
}

impl State for StateIdle {
    fn next(&self) -> (Box<dyn State>, bool, bool) {
        (
            Box::new(StateIdle {
                up: self.up,
                down: self.down,
            }),
            self.up,
            self.down,
        )
    }
}

pub struct Blind {
    state: Box<dyn State>,
}

impl Blind {
    pub fn new() -> Self {
        Blind {
            state: Box::new(StateIdle {
                up: false,
                down: false,
            }),
        }
    }

    pub fn update(&mut self, _now: Instant, _in1: bool, _in2: bool) -> (bool, bool) {
        let (newstate, up, down) = self.state.next();
        self.state = newstate;
        (up, down)
    }
}
