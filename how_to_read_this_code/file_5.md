How do I read this in Rust?

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```


To read the provided code snippet in Rust:

1. Struct Definition:
   - The code defines a struct named `Guess`.
   - The struct has one field named `value` of type `i32`.

2. Implementation Block:
   - The code includes an `impl` block for the `Guess` struct.
   - Within the `impl` block, there is a `new` associated function defined for the `Guess` struct.
   - The `new` function takes a parameter named `value` of type `i32`.
   - If the `value` is less than 1 or greater than 100, the function panics with a custom error message.
   - Otherwise, it creates a new `Guess` struct instance by using field initialization shorthand.

3. Test Module:
   - After the struct definition and implementation block, there is a module declaration for tests indicated by `#[cfg(test)]`.
   - This module is only compiled and executed when running tests.
   - The `mod tests` statement declares a submodule named `tests` within the current module.

4. Test Function:
   - Within the `tests` submodule, there is a test function named `greater_than_100`.
   - The `#[test]` attribute indicates that this is a unit test.
   - The test function calls the `Guess::new` associated function with the argument `200`.
   - The `#[should_panic]` attribute is used to indicate that this test is expected to panic (raise a panic).
   - If the `Guess::new` function does not panic when called with `200`, the test will fail.

In this code, the `Guess` struct represents a guess with a value between 1 and 100. The `new` function is responsible for creating a new `Guess` instance and panics if the provided `value` is outside the valid range. The test function verifies that calling `Guess::new` with a value greater than 100 triggers a panic as expected.

You can run the test by executing `cargo test` in the project directory to ensure that the `Guess::new` function correctly handles out-of-range values and raises a panic.