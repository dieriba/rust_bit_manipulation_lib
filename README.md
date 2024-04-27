# bit_manipulation-rs

A Rust library for efficient manipulation of 128-bit integers at the bit level.

## Features

- **Flexible Bit Manipulation:** Perform various operations on 128-bit integers including setting, clearing, and checking individual bits.
  
- **Efficient Implementation:** Utilizes bitwise operations for high-performance bit manipulation.
  
- **Intuitive API:** Provides a straightforward API for working with 128-bit integers, making it easy to integrate into your Rust projects.

## Functions
### new() -> Bit128
Creates a new Bit128 instance with all bits initially set to 0.
### is_bit_on(bit: u8) -> bool

Checks if the specified bit is on (1) or off (0). Returns true if the bit is set, false otherwise.

### are_bits_on(bits: Vec<u8>) -> Vec<bool>

Checks if multiple specified bits are on (1) or off (0). Returns a vector of booleans indicating whether each bit is set or not.

###  set_bit(bit: u8)
Sets the specified bit to 1. Requires bit to be less than 128.


### set_bits(bits: Vec<u8>)

Sets multiple specified bits to 1. Requires all bits to be less than 128.

###  clear_bit(bit: u8)

Clears the specified bit, setting it to 0. Requires bit to be less than 128.

### clear_bits(bits: Vec<u8>)

Clears multiple specified bits, setting them to 0. Requires all bits to be less than 128.
### clear_all_bit128()

Clears all bits, setting them to 0.

### get_value() -> u128

Returns the current value of the 128-bit integer.
### get_all_bit128() -> &Vec<bool>

Returns a reference to the vector representing the state of all bits.
### set_all_flags()

Sets all bits to 1, effectively setting the value to the maximum possible value of a u128.

## Usage

```rust
use bit_manipulation::Bit128;

fn main() {
    // Create a new Bit128 instance
    let mut bit128 = Bit128::new();

    // Set a bit
    bit128.set_bit(5);

    // Check if a bit is on
    assert_eq!(bit128.is_bit_on(5), true);

    // Clear a bit
    bit128.clear_bit(5);

    // Check if a bit is off
    assert_eq!(bit128.is_bit_on(5), false);
}
```
## Support
You can contact me at dieriba.pro@gmail.com, for any additional requests or features you wand to add !