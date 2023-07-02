# Full code Explanation 19-22

Demonstration:

- how traits can be used to define common behavior across different types
- how multiple traits can be implemented for a single struct, providing more functionality and flexibility

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let point = Point { x: 3, y: 7 };
    point.outline_print();
}
```

___


The given code demonstrates the use of traits, trait inheritance, and implementing the `fmt::Display` trait for a custom struct in Rust. Let's break down how the code works in detail:

1. Importing the `fmt` module:
   - The code begins with importing the `fmt` module from the standard library. This module provides types and traits for formatting and displaying output.

2. Defining the `OutlinePrint` trait:
   - The code defines a custom trait named `OutlinePrint`. This trait inherits from the `fmt::Display` trait, which means any type implementing `OutlinePrint` must also implement `fmt::Display`.
   - The `OutlinePrint` trait defines a single function named `outline_print`, which takes a reference to `self` and doesn't return any value. This function is responsible for printing an outlined representation of the implementing type.

3. Defining the `Point` struct:
   - The code defines a custom struct named `Point` with two fields, `x` and `y`, both of type `i32`.

4. Implementing `OutlinePrint` for `Point`:
   - The code implements the `OutlinePrint` trait for the `Point` struct using the `impl` block and `for` keyword. This means that instances of `Point` can use the `outline_print` function defined in the trait.
   - Since `OutlinePrint` inherits from `fmt::Display`, `Point` must also implement `fmt::Display`. This requirement is satisfied later in the code.

5. Implementing `fmt::Display` for `Point`:
   - The code implements the `fmt::Display` trait for the `Point` struct using another `impl` block.
   - The `fmt` function is defined, which takes a reference to `self` and a reference to a `fmt::Formatter`. It formats the `Point` struct as a string in the format "(x, y)" using the `write!` macro.

6. The `main` function:
   - The `main` function is left empty in this code snippet.

Now, let's understand the purpose of the code:

The code defines a trait `OutlinePrint`, which provides a default implementation of `outline_print` for types that implement `fmt::Display`. The `outline_print` function generates an outlined representation of the implementing type by printing the output surrounded by asterisks.

The `Point` struct implements both `OutlinePrint` and `fmt::Display`. The implementation of `OutlinePrint` for `Point` allows instances of `Point` to use the `outline_print` function. The implementation of `fmt::Display` enables the `Point` struct to be formatted as a string in the format "(x, y)".

In the given code, the `main` function is empty and doesn't execute any code. To observe the behavior, you could create a `Point` instance and call the `outline_print` function to see the outlined representation printed to the console.

Example usage in `main`:
```rust
fn main() {
    let point = Point { x: 3, y: 7 };
    point.outline_print();
}
```

The output would be:
```
**********
*        *
* (3, 7) *
*        *
**********
```

Overall, the code demonstrates how traits can be used to define common behavior across different types and how multiple traits can be implemented for a single struct, providing more functionality and flexibility.