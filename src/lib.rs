mod private {
    pub use std::ops::*;
    pub const BYTES: usize = 8;

    pub trait Uint:
        std::fmt::Display
        + BitAnd<Self, Output = Self>
        + BitOr<Self, Output = Self>
        + BitAndAssign<Self>
        + BitOrAssign<Self>
        + Not<Output = Self>
        + std::cmp::PartialOrd<Self>
        + Copy
        + From<u8>
    {
        fn max_value() -> Self;
        fn in_memory_size() -> u8;
        fn convert(bit: u8) -> Self;
    }
}

impl private::Uint for u8 {
    fn max_value() -> u8 {
        std::u8::MAX
    }
    fn in_memory_size() -> u8 {
        (std::mem::size_of::<u8>() * private::BYTES) as u8
    }
    fn convert(bit: u8) -> Self {
        1 << bit
    }
}

impl private::Uint for u16 {
    fn max_value() -> u16 {
        std::u16::MAX
    }
    fn in_memory_size() -> u8 {
        (std::mem::size_of::<u16>() * private::BYTES) as u8
    }
    fn convert(bit: u8) -> Self {
        1 << bit
    }
}

impl private::Uint for u32 {
    fn max_value() -> u32 {
        std::u32::MAX
    }
    fn in_memory_size() -> u8 {
        (std::mem::size_of::<u32>() * private::BYTES) as u8
    }
    fn convert(bit: u8) -> Self {
        1 << bit
    }
}

impl private::Uint for u64 {
    fn max_value() -> u64 {
        std::u64::MAX
    }
    fn in_memory_size() -> u8 {
        (std::mem::size_of::<u64>() * private::BYTES) as u8
    }
    fn convert(bit: u8) -> Self {
        1 << bit
    }
}

impl private::Uint for u128 {
    fn max_value() -> u128 {
        std::u128::MAX
    }
    fn in_memory_size() -> u8 {
        (std::mem::size_of::<u128>() * private::BYTES) as u8
    }
    fn convert(bit: u8) -> Self {
        1 << bit
    }
}

pub mod bit_manipulation {
    use crate::private;

    #[derive(Default, Debug)]
    pub struct Bits<T>
    where
        T: private::Uint,
    {
        value: T,
        activated_bits: Vec<bool>,
        size: u8,
    }

    impl<T> Bits<T>
    where
        T: private::Uint,
    {
        pub fn new() -> Self {
            Bits {
                value: T::from(0),
                size: T::in_memory_size(),
                activated_bits: vec![false; T::in_memory_size() as usize],
            }
        }

        pub fn is_bit_on(&self, bit: u8) -> bool {
            if bit >= self.size {
                return false;
            }

            self.value & T::convert(bit) > T::from(0)
        }

        pub fn are_bits_on(&self, bits: &Vec<u8>) -> Vec<bool> {
            let mut activated_flags = vec![false; self.size as usize];
            for &bit in bits {
                if self.is_bit_on(bit) {
                    activated_flags[bit as usize] = true;
                }
            }
            activated_flags
        }

        pub fn set_bit(&mut self, bit: u8) -> bool {
            if bit >= self.size {
                return false;
            }
            self.value |= T::convert(bit);
            self.activated_bits[bit as usize] = true;
            true
        }

        pub fn set_bits(&mut self, bits: &Vec<u8>) -> &Vec<bool> {
            for &bit in bits {
                self.set_bit(bit);
            }
            &self.activated_bits
        }

        pub fn clear_bit(&mut self, bit: u8) -> bool {
            if bit >= self.size {
                return false;
            }
            self.value &= !(T::convert(bit));
            self.activated_bits[bit as usize] = false;
            true
        }

        pub fn clear_bits(&mut self, bits: &Vec<u8>) -> &Vec<bool> {
            for &bit in bits {
                self.clear_bit(bit);
            }
            &self.activated_bits
        }

        pub fn clear_all_bits(&mut self) {
            self.value = T::from(0);
        }

        pub fn get_value(&self) -> T {
            self.value
        }

        pub fn get_all_bits(&self) -> &Vec<bool> {
            &self.activated_bits
        }

        pub fn set_all_flags(&mut self) {
            self.value = T::max_value();
        }

        pub fn get_in_size_memory(&self) -> u8 {
            self.size
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bit_manipulation::Bits;

    use super::*;

    #[test]
    fn set_bit() {
        let mut bits: bit_manipulation::Bits<u32> = bit_manipulation::Bits::new();
        let mut bit = 0;

        bits.set_bit(bit);
        assert!(bits.is_bit_on(bit));

        bit = 6;
        bits.set_bit(bit);
        assert!(bits.is_bit_on(bit));

        bit = 15;
        bits.set_bit(bit);
        assert!(bits.is_bit_on(bit));

        let mut bits: bit_manipulation::Bits<u64> = bit_manipulation::Bits::new();

        bit = 63;
        bits.set_bit(bit);
        assert!(bits.is_bit_on(bit));

        bit = 66;
        bits.set_bit(bit);
        assert!(!bits.is_bit_on(bit));

        let mut bits: bit_manipulation::Bits<u128> = bit_manipulation::Bits::new();

        bit = 0;
        bits.set_bit(bit);
        assert!(bits.is_bit_on(bit));

        bit = 127;
        bits.set_bit(bit);
        assert!(bits.is_bit_on(bit));
    }

    #[test]
    fn set_bits() {
        // Create a new bit set for u64 integers
        let mut bits: Bits<u8> = bit_manipulation::Bits::new();
        let arr = vec![1, 3, 5, 2, 65];
        // Set multiple bits
        let res = bits.set_bits(&arr);

        // Check if specific bits are on
        assert_eq!(
            res,
            &vec![false, true, true, true, false, true, false, false]
        );

        // Clear all bits
        bits.clear_all_bits();

        // Check if all bits are cleared
        assert_eq!(bits.get_value(), 0);
    }

    #[test]
    fn clear_bit() {
        let mut bits: Bits<u8> = bit_manipulation::Bits::new();

        let set_success = bits.set_bit(10);

        assert!(!bits.is_bit_on(10));

        assert!(!set_success);

        let clear_success = bits.clear_bit(10);

        assert!(!bits.is_bit_on(10));

        assert!(!clear_success);

        assert_eq!(bits.get_value(), 0);
    }

    #[test]
    fn clear_bits() {
        // Create a new bit set for u8 integers
        let mut bits: Bits<u8> = bit_manipulation::Bits::new();
        let arr = vec![1, 3, 5, 2, 65];
        // Set multiple bits
        let mut activated_bits = bits.set_bits(&arr);

        assert_eq!(
            activated_bits,
            &vec![false, true, true, true, false, true, false, false]
        );

        activated_bits = bits.clear_bits(&arr);

        assert_eq!(
            activated_bits,
            &vec![false, false, false, false, false, false, false, false]
        );

        // Check if all bits are cleared
        assert_eq!(bits.get_value(), 0);
    }

    #[test]
    fn test_in_size_memory() {
        let bits: Bits<u8> = bit_manipulation::Bits::new();
        assert_eq!(8, bits.get_in_size_memory());
        let bits: Bits<u16> = bit_manipulation::Bits::new();
        assert_eq!(16, bits.get_in_size_memory());
        let bits: Bits<u32> = bit_manipulation::Bits::new();
        assert_eq!(32, bits.get_in_size_memory());
        let bits: Bits<u64> = bit_manipulation::Bits::new();
        assert_eq!(64, bits.get_in_size_memory());
        let bits: Bits<u128> = bit_manipulation::Bits::new();
        assert_eq!(128, bits.get_in_size_memory());
    }

    #[test]
    fn test_max_size() {
        let mut bits: Bits<u8> = bit_manipulation::Bits::new();
        bits.set_all_flags();
        assert_eq!(u8::MAX, bits.get_value());
        let mut bits: Bits<u16> = bit_manipulation::Bits::new();
        bits.set_all_flags();
        assert_eq!(u16::MAX, bits.get_value());
        let mut bits: Bits<u32> = bit_manipulation::Bits::new();
        bits.set_all_flags();
        assert_eq!(u32::MAX, bits.get_value());
        let mut bits: Bits<u64> = bit_manipulation::Bits::new();
        bits.set_all_flags();
        assert_eq!(u64::MAX, bits.get_value());
        let mut bits: Bits<u128> = bit_manipulation::Bits::new();
        bits.set_all_flags();
        assert_eq!(u128::MAX, bits.get_value());
    }
}
