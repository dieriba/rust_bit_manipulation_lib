pub mod bit_manipulation {

    pub const SIZE: u8 = 128;

    #[derive(Default)]
    pub struct Options {
        value: u128,
        activated_options: Vec<bool>,
    }

    impl Options {
        pub fn new() -> Self {
            Options {
                value: 0,
                activated_options: vec![false; SIZE as usize],
            }
        }

        pub fn check_bit(&self, bit: u8) -> bool {
            if bit >= SIZE {
                return false;
            }

            self.value & (1 << bit) > 0
        }

        pub fn check_bits(&self, bits: Vec<u8>) -> Vec<bool> {
            let mut options = vec![false; SIZE as usize];
            for &bit in &bits {
                if self.check_bit(bit) {
                    options[bit as usize] = true;
                }
            }
            options
        }

        pub fn set_bit(&mut self, bit: u8) {
            if bit >= SIZE {
                return ;
            }
            self.value |= 1 << bit;
            self.activated_options[bit as usize] = true;
        }

        pub fn set_bits(&mut self, bits: Vec<u8>) {
            for &bit in &bits {
                if bit < SIZE {
                    self.value |= 1 << bit;
                    self.activated_options[bit as usize] = true;
                }
            }
        }

        pub fn clear_bit(&mut self, bit: u8) {
            if bit >= SIZE {
                return ;
            }
            self.value &= !(1 << bit);
            self.activated_options[bit as usize] = false;
        }

        pub fn clear_bits(&mut self, bits: Vec<u8>) {
            for &bit in &bits {
                if bit >= SIZE {
                    self.value &= !(1 << bit);
                    self.activated_options[bit as usize] = false;
                }
            }
        }

        pub fn clear_all_options(&mut self) {
            self.value = 0;
        }

        pub fn get_value(&self) -> u128 {
            self.value
        }

        pub fn get_all_options(&self) -> &Vec<bool> {
            &self.activated_options
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
        let mut options = bit_manipulation::Options::new();
        
        let mut bit = 0;
        options.set_bit(bit);
        assert!(options.check_bit(bit));

        bit = 4;
        options.set_bit(bit);
        assert!(options.check_bit(bit));

        bit = 7;
        options.set_bit(bit);
        assert!(options.check_bit(bit));

        bit = 127;
        options.set_bit(bit);
        assert!(options.check_bit(bit));
    }
    #[test]
    fn clear_bit() {
        let mut options = bit_manipulation::Options::new();
        options.set_all_flags();
        let mut bit = 0;
        options.clear_bit(bit);
        assert!(!options.check_bit(bit));

        bit = 4;
        options.clear_bit(bit);
        assert!(!options.check_bit(bit));

        bit = 7;
        options.clear_bit(bit);
        assert!(!options.check_bit(bit));

        bit = 127;
        options.clear_bit(bit);
        assert!(!options.check_bit(bit));
    }
}