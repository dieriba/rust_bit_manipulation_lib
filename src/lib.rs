pub mod bit_manipulation {

    pub const SIZE: u8 = 128;

    #[derive(Default)]
    pub struct Bit128 {
        value: u128,
        activated_bit128: Vec<bool>,
    }

    impl Bit128 {
        pub fn new() -> Self {
            Bit128 {
                value: 0,
                activated_bit128: vec![false; SIZE as usize],
            }
        }

        pub fn is_bit_on(&self, bit: u8) -> bool {
            if bit >= SIZE {
                return false;
            }

            self.value & (1 << bit) > 0
        }

        pub fn are_bits_on(&self, bits: Vec<u8>) -> Vec<bool> {
            let mut bit128 = vec![false; SIZE as usize];
            for &bit in &bits {
                if self.is_bit_on(bit) {
                    bit128[bit as usize] = true;
                }
            }
            bit128
        }

        pub fn set_bit(&mut self, bit: u8) {
            if bit >= SIZE {
                return ;
            }
            self.value |= 1 << bit;
            self.activated_bit128[bit as usize] = true;
        }

        pub fn set_bits(&mut self, bits: Vec<u8>) {
            for &bit in &bits {
                if bit < SIZE {
                    self.value |= 1 << bit;
                    self.activated_bit128[bit as usize] = true;
                }
            }
        }

        pub fn clear_bit(&mut self, bit: u8) {
            if bit >= SIZE {
                return ;
            }
            self.value &= !(1 << bit);
            self.activated_bit128[bit as usize] = false;
        }

        pub fn clear_bits(&mut self, bits: Vec<u8>) {
            for &bit in &bits {
                if bit >= SIZE {
                    self.value &= !(1 << bit);
                    self.activated_bit128[bit as usize] = false;
                }
            }
        }

        pub fn clear_all_bit128(&mut self) {
            self.value = 0;
        }

        pub fn get_value(&self) -> u128 {
            self.value
        }

        pub fn get_all_bit128(&self) -> &Vec<bool> {
            &self.activated_bit128
        }
        
        pub fn set_all_flags(&mut self) {
            self.value = std::u128::MAX;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_bit() {
        let mut bit128 = bit_manipulation::Bit128::new();
        
        let mut bit = 0;
        bit128.set_bit(bit);
        assert!(bit128.is_bit_on(bit));

        bit = 4;
        bit128.set_bit(bit);
        assert!(bit128.is_bit_on(bit));

        bit = 7;
        bit128.set_bit(bit);
        assert!(bit128.is_bit_on(bit));

        bit = 127;
        bit128.set_bit(bit);
        assert!(bit128.is_bit_on(bit));
    }
    #[test]
    fn clear_bit() {
        let mut bit128 = bit_manipulation::Bit128::new();
        bit128.set_all_flags();
        let mut bit = 0;
        bit128.clear_bit(bit);
        assert!(!bit128.is_bit_on(bit));

        bit = 4;
        bit128.clear_bit(bit);
        assert!(!bit128.is_bit_on(bit));

        bit = 7;
        bit128.clear_bit(bit);
        assert!(!bit128.is_bit_on(bit));

        bit = 127;
        bit128.clear_bit(bit);
        assert!(!bit128.is_bit_on(bit));
    }
}