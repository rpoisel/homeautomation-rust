#[cfg(test)]
mod blind_tests {
    use crate::automation::*;
    use std::time;

    #[test]
    fn test_state_change_idle_to_up() {
        let mut blind = Blind::new();
        assert_eq!(blind.state(), "idle");
        blind.update(time::Instant::now(), true, true);
        assert_eq!(blind.state(), "idle");
    }
}
