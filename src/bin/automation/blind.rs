use std::time::Instant;

trait State {
    fn next(self: Box<Self>, _up: bool, _down: bool) -> (Box<dyn State>, bool, bool);
    fn name(&self) -> &'static str;
}

struct StateIdle {
    // up: bool,
// down: bool,
}

impl State for StateIdle {
    fn next(self: Box<Self>, _up: bool, _down: bool) -> (Box<dyn State>, bool, bool) {
        // Box::new(StateIdle {
        //     up: self.up,
        //     down: self.down,
        // }),

        // self, self.up, self.down,
        (self, false, false)
    }
    fn name(&self) -> &'static str {
        "idle"
    }
}

/// A representation of a window blind
///
/// Directly forward states of inputs into instances using the update method.
/// Any edge detection will be performed within the blind instances themselves.
pub struct Blind {
    state: Option<Box<dyn State>>,
}

impl Blind {
    pub fn new() -> Self {
        Blind {
            state: Some(Box::new(StateIdle {})),
        }
    }

    pub fn update(&mut self, _now: Instant, up: bool, down: bool) -> Option<(bool, bool)> {
        if let Some(s) = self.state.take() {
            let (nextstate, new_up, new_down) = s.next(up, down);
            self.state = Some(nextstate);
            return Some((new_up, new_down));
        }
        None
    }

    // Currently, this method only exists for tests
    // I simply don't know how to do it any better yet :-)
    pub fn state(&self) -> &'static str {
        match &self.state {
            Some(s) => s.name(),
            None => "none",
        }
    }
}
