pub trait BitOp {
    fn bit_is_set(&self, pos: usize) -> bool;
}

impl BitOp for u8 {
    fn bit_is_set(&self, pos: usize) -> bool {
        self & (0x01 << pos) == (0x01 << pos)
    }
}

pub struct BitValue<T: BitOp> {
    pub val: T,
}

#[cfg(test)]
mod tests {
    use crate::bitop::*;

    #[test]
    fn low_bound_works() {
        assert_eq!(BitValue { val: 0 }.val.bit_is_set(0), false);
    }

    #[test]
    fn high_bound_works() {
        assert_eq!(BitValue { val: 255 }.val.bit_is_set(7), true);
    }

    #[test]
    fn mid_pos_works() {
        assert_eq!(BitValue { val: 0b0001_0000 }.val.bit_is_set(5), false);
        assert_eq!(BitValue { val: 0b0001_0000 }.val.bit_is_set(4), true);
        assert_eq!(BitValue { val: 0b0001_0000 }.val.bit_is_set(3), false);
    }

    #[test]
    #[should_panic(expected = "attempt to shift left with overflow")]
    fn high_bound_plus_one_panics() {
        BitValue { val: 255 }.val.bit_is_set(8);
    }
}
