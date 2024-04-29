# bit_manipulation-rs

A Rust library for manipulating unsigned integers from u8 to u128, allowing you to set, clear, or retrieve individual bits.

## Features

- **Bit Manipulation**: Set, clear, and retrieve individual bits within unsigned integers.

- **Cross-Type Support**: Works with `u8`, `u16`, `u32`, `u64`, and `u128` types.

- **Efficient Storage**: Handles bit manipulation efficiently by storing bits as boolean flags.

- **Efficient Implementation:** Utilizes bitwise operations for high-performance bit manipulation.
  
- **Intuitive API:** Provides a straightforward API for working with u8..u128, making it easy to integrate into your Rust projects.

## Functions
### new() -> Bits
Creates a new Bits instance with all bits initially set to 0.
### is_bit_on(bit: u8) -> bool

Checks if the specified bit is on (1) or off (0).  
#### Returns
- true if the bit is set, false otherwise.

### are_bits_on(bits: Vec<u8>) -> Vec<bool>

Checks if multiple specified bits are on (1) or off (0).  
> **Note:**  
> Return a vector of booleans indicating whether each bit is set (`true`) or unset (`false`).  
The vector will contain the state of all bits within the integer value, not just the ones specified in the `bits` parameter.

###  set_bit(bit: u8) -> bool
Sets the specified bit within the unsigned integer value represented by the Bits struct.  
The bit parameter indicates the position of the bit to set, starting from 0 for the least significant bit.

> **Note:**  
> The maximum value that the bit parameter can have depends on the base type T.  
For example, if T is u8, the maximum valid value for bit would be 7 (since u8 has 8 bits, indexed from 0 to 7).   
Attempting to set a bit with a value higher than the maximum valid index will result in the function returning false without modifying the state of the Bits struct.  

#### Returns
- true if the operation succeeded, false otherwise.

### set_bits(bits: Vec<u8>) -> &Vec<bool>

Sets the multiple specified bits within the unsigned integer value represented by the Bits struct.  

> **Note:**  
> The maximum value that the each bit can have depends on the base type T.  
For example, if T is u8, the maximum valid value for bit would be 7 (since u8 has 8 bits, indexed from 0 to 7).   
Attempting to set a bit with a value higher than the maximum valid index will result in the function to ignore and continue to set valid bits.

> **Note:**  
> Return a vector of boolean indicating whether each bit is set (`true`) or unset (`false`).  
The vector will contain the state of all bits within the integer value, not just the ones specified in the `bits` parameter.

###  clear_bit(bit: u8) -> bool

Clears the specified bit, setting it to 0.  
#### Returns
- true if the operation succeeded, false otherwise.

### clear_bits(bits: Vec<u8>)

Clears the specified bits in a value of type `T`, where `T` is expected to be one of:
`u8`, `u16`, `u32`, `u64`, or `u128`. Only the bits that are within the range of the maximum
value of type `T` will be cleared. For example, if the base type is `u8` and the method
is called with the vector `[1, 5, 10]`, only the bits at positions 1 and 5 will be cleared,
since `u8` does not have a 10th bit.
### clear_all_bits()

Clears all bits, setting them to 0.

### get_value() -> T

#### Returns
- the current value of the T unsigned integer.
### get_all_bits() -> &Vec<bool>

#### Returns
- A reference to the vector representing the state of all bits.
### set_all_flags()

Sets all bits to 1, effectively setting the value to the maximum possible value of given type T.

## Usage

```rust
use bit_manipulation::Bits;

fn main() {
    // Create a new bit set for u8 integers
    let mut bits: Bits<u8> = bit_manipulation::Bits::new();

    // Try to set a bit that exceeds the size of the integer
    let set_success = bits.set_bit(10);

    // Check if the bit is on (should be false)
    assert!(!bits.is_bit_on(10));
    
    // Check if the operation was successful
    assert!(!set_success);

    // Try to clear a bit that exceeds the size of the integer
    let clear_success = bits.clear_bit(10);

    // Check if the bit is off (should be false)
    assert!(!bits.is_bit_on(10));
    
    // Check if the operation was successful
    assert!(!clear_success);
}

```
```rust
use bit_manipulation::bit_manipulation::Bits;

fn main() {
    // Create a new bit set for u8 integers
    let mut bits: Bits<u8> = bit_manipulation::Bits::new();

    // Try to set a bit that exceeds the size of the integer
    let operation_success = bits.set_bit(10);

    // Check if the bit is on (should be false)
    assert!(!bits.is_bit_on(10));

    // Check if the operation was successful
    assert!(!operation_success);
}
```
```rust
use bit_manipulation::bit_manipulation::Bits;

fn main() {
    // Create a new bit set for u8 integers
    let mut bits: Bits<u8> = bit_manipulation::Bits::new();
    let arr = vec![1, 3, 5, 2, 65];

    // Set multiple bits
    bits.set_bits(arr);

    // Check if specific bits are on
    let activated_bits = bits.are_bits_on(arr);
    assert_eq!(activated_bits, vec![false, true, true, true, false, true, false, false]);

    // Clear all bits
    bits.clear_all_bits();

    // Check if all bits are cleared
    assert_eq!(bits.get_value(), 0);
}
```

## Support
You can contact me at dieriba.pro@gmail.com, for any additional requests or features you wand to add !