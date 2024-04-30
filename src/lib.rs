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
        /// Creates a new instance of the `Bits` struct with an initial value of zero.
        /// 
        /// This method constructs a new instance of the `Bits` struct, initializing the stored value 
        /// to zero and determining the size of the value based on the memory size of type `T`. It also 
        /// initializes a vector of boolean flags (`activated_bits`) to track the activation status of 
        /// individual bits within the stored value.
        /// 
        /// # Returns
        /// 
        /// A new instance of the `Bits` struct with the initial value set to zero.
        /// 
        /// # Example
        /// 
        /// ```
        /// let bits: Bits<u8> = Bits::new();
        /// // Creates a new instance of `Bits` with an initial value of zero and size based on `u8`.
        /// ```
        /// 
        /// This example demonstrates how to use the `new` method to create a new instance of the `Bits` 
        /// struct with default values.

        pub fn new() -> Self {
            Bits {
                value: T::from(0),
                size: T::in_memory_size(),
                activated_bits: vec![false; T::in_memory_size() as usize],
            }
        }

        /// Checks if a specific bit is set (activated) within the value stored in the struct instance.
        /// 
        /// This method determines whether the bit at the specified position `bit` is set (activated) 
        /// within the binary representation of the value stored in the struct instance. If the provided 
        /// bit position is greater than or equal to the size of the value, indicating it's out of range, 
        /// the method returns `false`.
        /// 
        /// # Parameters
        /// 
        /// - `bit`: The position of the bit to check within the value, ranging from 0 to `n - 1`(for example for u8 where n = 8) it would have to be between 0 and 7.
        /// 
        /// # Returns
        /// 
        /// - `true` if the specified bit is set (activated).
        /// - `false` if the provided bit position is out of range (greater than or equal to `self.size`) 
        ///   or if the bit is not set.
        /// # Example
        /// 
        /// ```
        /// let bits: Bits<u8> = Bits::new();
        /// // Creates a new instance of `Bits` with an initial value of zero and size based on `u8`.
        /// assert!(bits.is_bit_on(2));
        /// //Checking that the third bit is activated
        /// ```
        /// 
        /// This example demonstrates how to use the `new` method to create a new instance of the `Bits` 
        /// struct with default values.

        pub fn is_bit_on(&self, bit: u8) -> bool {
            if bit >= self.size {
                return false;
            }

            self.value & T::convert(bit) > T::from(0)
        }

        /// Checks if specific bits are set (activated) within the value stored in the struct instance.
        /// 
        /// This method examines whether the bits at the positions specified in the provided vector `bits` 
        /// are set (activated) within the binary representation of the value stored in the struct instance. 
        /// It returns a vector of boolean values indicating whether each specified bit is set (`true`) or 
        /// not (`false`). If a provided bit position is out of range (greater than or equal to `self.size`), 
        /// it is considered as not activated.
        /// 
        /// # Parameters
        /// 
        /// - `bits`: A reference to a vector containing the positions of the bits to check within the value.
        /// 
        /// # Returns
        /// 
        /// A vector of boolean values indicating whether each specified bit in the stored value is set 
        /// (`true`) or not (`false`).
        ///
        /// # Example
        /// 
        /// ```
        /// let bits: Bits<u8> = Bits::new();
        /// // Creates a new instance of `Bits` with an initial value of zero and size based on `u8`.
        /// let positions_to_check = vec![2, 4, 6];
        /// let activated_flags = bits.are_bits_on(&positions_to_check);
        /// // Checks if bits at positions 2, 4, and 6 are set.
        /// assert_eq!(activated_flags, vec![false, false, false, true, false, true, false, false]);
        /// // Only the bits at positions 4 and 6 are set in the stored value.
        /// ```
        /// 
        /// This example demonstrates how to use the `are_bits_on` method to check if specific bits are set within the stored value.


        pub fn are_bits_on(&mut self, bits: &Vec<u8>) -> &Vec<bool> {
            for &bit in bits {
                if self.is_bit_on(bit) {
                    self.activated_bits[bit as usize] = true;
                }
            }
            &self.activated_bits
        }

        /// Sets a specific bit in the value stored within the struct instance.
        /// 
        /// This method sets the bit at the specified position within the binary representation 
        /// of the value stored in the struct instance. If the provided bit position is greater 
        /// than or equal to the size of the value, indicating it's out of range, no action is 
        /// taken, and the method returns `false`. Otherwise, the specified bit is set, and the 
        /// method returns `true` to indicate that the operation was successful.
        /// 
        /// # Parameters
        /// 
        /// - `bit`: The position of the bit to set within the value, ranging from 0 to `n - 1`(for example u8 where n = 8)..
        /// 
        /// # Returns
        /// 
        /// - `true` if the specified bit was successfully set.
        /// - `false` if the provided bit position is out of range (greater than or equal to `self.size`).
        /// # Example
        /// 
        /// ```
        /// let mut bits: Bits<u8> = Bits::new();
        /// // Creates a new instance of `Bits` with an initial value of zero and size based on `u8`.
        /// let bit_to_set = 2;
        /// let set_success = bits.set_bit(bit_to_set);
        /// // Sets the bit at position 2 within the stored value.
        /// assert!(set_success);
        /// // Confirms that the bit at position 2 is set.
        /// assert!(bits.is_bit_on(bit_to_set));
        /// ```
        /// 
        /// This example demonstrates how to use the `set_bit` method to set a specific bit within the stored value.


        pub fn set_bit(&mut self, bit: u8) -> bool {
            if bit >= self.size {
                return false;
            }
            self.value |= T::convert(bit);
            self.activated_bits[bit as usize] = true;
            true
        }

        /// Sets multiple bits specified by their positions within the value stored in the struct instance.
        /// 
        /// This method sets each bit at the positions specified in the provided vector `bits` within 
        /// the binary representation of the value stored in the struct instance. If any specified bit 
        /// position is greater than or equal to the maximum bit position that the type `T` can have,
        /// it will be set to false and skipped. After setting all the specified bits, it returns a reference to the
        /// vector `activated_bits`, indicating which bits have been successfully set.
        /// 
        /// # Parameters
        /// 
        /// - `bits`: A reference to a vector containing the positions of the bits to set within the value.
        /// 
        /// # Returns
        /// 
        /// A reference to a vector containing boolean values indicating whether each corresponding bit 
        /// in the stored value has been successfully set (`true`) or not (`false`).
        /// # Example
        ///
        /// ```
        /// let mut bits: Bits<u8> = Bits::new();
        /// // Creates a new instance of `Bits` with an initial value of zero and size based on `u8`.
        /// let positions_to_set = vec![2, 4, 6];
        /// let activated_flags = bits.set_bits(&positions_to_set);
        /// // Sets the bits at positions 2, 4, and 6 within the stored value.
        /// assert_eq!(activated_flags, &vec![false, false, true, false, true, false, true, false]);
        /// // The returned vector indicates which bits were successfully set.
        /// ```
        /// 
        /// This example demonstrates how to use the `set_bits` method to set multiple bits within the stored value.

        pub fn set_bits(&mut self, bits: &Vec<u8>) -> &Vec<bool> {
            for &bit in bits {
                self.set_bit(bit);
            }
            &self.activated_bits
        }

        /// Clears a specific bit in the value stored within the struct instance.
        /// 
        /// This method clears the bit at the specified position within the binary representation 
        /// of the value stored in the struct instance. If the provided bit position is greater than
        /// or equal to the size of the value, no action is taken, and the method returns `false`.
        /// Otherwise, the specified bit is cleared, and the method returns `true` to indicate that 
        /// the operation was successful.
        /// 
        /// # Parameters
        /// 
        /// - `bit`: The position of the bit to clear within the value, ranging from 0 to `n - 1`(for example u8 where n = 8).
        /// 
        /// # Returns
        /// 
        /// - `true` if the specified bit was successfully cleared.
        /// - `false` if the provided bit position is out of range (greater than or equal to `n`).
        /// # Example
        /// 
        /// ```
        /// let mut bits: Bits<u8> = Bits::new();
        /// // Creates a new instance of `Bits` with an initial value of zero and size based on `u8`.
        /// bits.set_bit(2);
        /// // Sets the bit at position 2 within the stored value.
        /// assert!(bits.is_bit_on(2));
        /// // Confirms that the bit at position 2 is set.
        /// let clear_success = bits.clear_bit(2);
        /// // Clears the bit at position 2 within the stored value.
        /// assert!(clear_success);
        /// // Confirms that the bit at position 2 is now cleared.
        /// assert!(!bits.is_bit_on(2));
        /// ```
        /// 
        /// This example demonstrates how to use the `clear_bit` method to clear a specific bit within the stored value.

        pub fn clear_bit(&mut self, bit: u8) -> bool {
            if bit >= self.size {
                return false;
            }
            self.value &= !(T::convert(bit));
            self.activated_bits[bit as usize] = false;
            true
        }
        /// Clears multiple bits specified by their positions within the value stored in the struct instance.
        /// 
        /// This method clears each bit at the positions specified in the provided vector `bits` within 
        /// the binary representation of the value stored in the struct instance. If any specified bit 
        /// position is greater than or equal to the size of the value, it is considered out of range
        /// and will be skipped. After clearing all the specified bits, it returns a reference to the
        /// vector `activated_bits`, indicating which bits have been successfully cleared.
        /// 
        /// # Parameters
        /// 
        /// - `bits`: A reference to a vector containing the positions of the bits to clear within the value.
        /// 
        /// # Returns
        /// 
        /// A reference to a vector containing boolean values indicating whether each corresponding bit 
        /// in the stored value has been successfully cleared (`false`) or not (`true`).
        /// 
        /// # Example
        /// 
        /// ```
        /// let mut bits: Bits<u8> = Bits::new();
        /// // Creates a new instance of `Bits` with an initial value of zero and size based on `u8`.
        /// bits.set_bit(2);
        /// bits.set_bit(4);
        /// bits.set_bit(6);
        /// // Sets the bits at positions 2, 4, and 6 within the stored value.
        /// assert_eq!(bits.get_value(), 84); // binary representation: 1010100
        /// let positions_to_clear = vec![2, 4, 6];
        /// let cleared_flags = bits.clear_bits(&positions_to_clear);
        /// // Clears the bits at positions 2, 4, and 6 within the stored value.
        /// assert_eq!(cleared_flags, &vec![false, false, false, false, false, false, false, false]);
        /// // The returned vector indicates which bits were successfully cleared.
        /// assert_eq!(bits.get_value(), 0); // After clearing, the value becomes 0.
        /// ```
        /// 
        /// This example demonstrates how to use the `clear_bits` method to clear multiple bits within the stored value.

        pub fn clear_bits(&mut self, bits: &Vec<u8>) -> &Vec<bool> {
            for &bit in bits {
                self.clear_bit(bit);
            }
            &self.activated_bits
        }

        /// Clears all bits, setting them to 0.
        /// 
        /// This method sets all bits within the value stored in the struct instance to 0.
        /// 
        /// # Example
        /// 
        /// ```
        /// let mut bits: Bits<u8> = Bits::new();
        /// // Creates a new instance of `Bits` with an initial value of zero and size based on `u8`.
        /// bits.set_bit(2);
        /// bits.set_bit(4);
        /// bits.set_bit(6);
        /// // Sets the bits at positions 2, 4, and 6 within the stored value.
        /// assert_eq!(bits.get_value(), 84); // binary representation: 1010100
        /// bits.clear_all_bits();
        /// // Clears all bits within the stored value.
        /// assert_eq!(bits.get_value(), 0); // After clearing, the value becomes 0.
        /// ```
        /// 
        /// This example demonstrates how to use the `clear_all_bits` method to clear all bits within the stored value.


        pub fn clear_all_bits(&mut self) {
            self.value = T::from(0);
        }

        /// Returns the current value of the unsigned integer type `T`.
        /// 
        /// This method returns the current value stored within the struct instance.
        /// 
        /// # Returns
        /// 
        /// The current value of the unsigned integer type `T`.
        /// 
        /// # Example
        /// 
        /// ```
        /// let mut bits: Bits<u8> = Bits::new();
        /// // Creates a new instance of `Bits` with an initial value of zero and size based on `u8`.
        /// bits.set_bit(2);
        /// bits.set_bit(4);
        /// bits.set_bit(6);
        /// // Sets the bits at positions 2, 4, and 6 within the stored value.
        /// assert_eq!(bits.get_value(), 84); // binary representation: 1010100
        /// ```
        /// 
        /// This example demonstrates how to use the `get_value` method to retrieve the current value stored within the `Bits` struct.

        pub fn get_value(&self) -> T {
            self.value
        }

        /// Returns a reference to the vector representing the state of all bits.
        /// 
        /// This method returns a reference to the vector `activated_bits`, which represents the state 
        /// of all bits within the value stored in the struct instance.
        /// 
        /// # Returns
        /// 
        /// A reference to a vector containing boolean values indicating the state of each bit 
        /// within the stored value.
        /// 
        /// # Example
        /// 
        /// ```
        /// let mut bits: Bits<u8> = Bits::new();
        /// // Creates a new instance of `Bits` with an initial value of zero and size based on `u8`.
        /// bits.set_bit(2);
        /// bits.set_bit(4);
        /// bits.set_bit(6);
        /// // Sets the bits at positions 2, 4, and 6 within the stored value.
        /// assert_eq!(bits.get_all_bits(), &vec![false, false, true, false, true, false, true, false]);
        /// ```
        /// 
        /// This example demonstrates how to use the `get_all_bits` method to retrieve the state of all bits within the stored value.


        pub fn get_all_bits(&self) -> &Vec<bool> {
            &self.activated_bits
        }
             
        /// Sets all bits to 1, effectively setting the value to the maximum possible value of the given type `T`.
        /// 
        /// This method sets all bits within the value stored in the struct instance to 1, effectively setting 
        /// the value to the maximum possible value of the given type `T`. The behavior varies based on the 
        /// maximum value representable by the type `T`.
        /// 
        /// # Example
        /// 
        /// ```
        /// let mut bits: Bits<u8> = Bits::new();
        /// // Creates a new instance of `Bits` with an initial value of zero and size based on `u8`.
        /// bits.set_all_flags();
        /// // Sets all bits within the stored value to 1, effectively setting it to the maximum possible value of `u8`.
        /// assert_eq!(bits.get_value(), 255); // Maximum value representable by `u8`.
        /// 
        /// let mut bits: Bits<u16> = Bits::new();
        /// bits.set_all_flags();
        /// assert_eq!(bits.get_value(), 65535); // Maximum value representable by `u16`.
        /// 
        /// let mut bits: Bits<u32> = Bits::new();
        /// bits.set_all_flags();
        /// assert_eq!(bits.get_value(), 4294967295); // Maximum value representable by `u32`.
        /// 
        /// let mut bits: Bits<u64> = Bits::new();
        /// bits.set_all_flags();
        /// assert_eq!(bits.get_value(), 18446744073709551615); // Maximum value representable by `u64`.
        /// 
        /// let mut bits: Bits<u128> = Bits::new();
        /// bits.set_all_flags();
        /// assert_eq!(bits.get_value(), 340282366920938463463374607431768211455); // Maximum value representable by `u128`.
        /// ```
        /// 
        /// This example demonstrates how to use the `set_all_flags` method to set all bits to 1, effectively 
        /// setting the value to the maximum possible value of the given type `T`.

        pub fn set_all_flags(&mut self) {
            self.value = T::max_value();
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
        let mut bits: Bits<u8> = bit_manipulation::Bits::new();
        let arr = vec![1, 3, 5, 2, 65];
        let res = bits.set_bits(&arr);

        assert_eq!(
            res,
            &vec![false, true, true, true, false, true, false, false]
        );

        bits.clear_all_bits();

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
        let mut bits: Bits<u8> = bit_manipulation::Bits::new();
        let arr = vec![1, 3, 5, 2, 65];
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

        assert_eq!(bits.get_value(), 0);
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
