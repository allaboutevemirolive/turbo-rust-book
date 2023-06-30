# What is deref coercion

What is deref coercion in rust? Give an example

___

Deref coercion is a feature in Rust that automatically converts a reference to a type implementing the `Deref` trait into a reference to the type it dereferences to. This coercion happens implicitly when certain types don't match exactly but have a compatible relationship through the `Deref` trait.

The `Deref` trait in Rust is used to overload the dereference operator (`*`). It allows types to customize the behavior of dereferencing, enabling convenient access to the data inside a smart pointer or wrapper type.

Here's an example to illustrate how deref coercion works:

```rust
struct MyString(String);

impl std::ops::Deref for MyString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn print_length(s: &str) {
    println!("Length: {}", s.len());
}

fn main() {
    let my_string = MyString(String::from("Hello, Rust!"));

    // The following line compiles and works due to deref coercion
    print_length(&my_string);
}
```

In this example, we define a custom type `MyString` that wraps a `String` inside it. We implement the `Deref` trait for `MyString` to specify that dereferencing a `MyString` should yield a reference to the `String` it contains.

The `print_length` function expects a reference to a `str` (a string slice) and prints its length. However, in the `main` function, we pass a reference to `my_string`, which is of type `&MyString`, not `&str`. Despite this mismatch, the code compiles and works correctly.

This is because Rust automatically applies deref coercion in this situation. When we call `print_length(&my_string)`, Rust recognizes that `&MyString` implements `Deref`, and therefore it can be implicitly converted to `&str`. The compiler inserts the necessary dereference (`*`) and function call (`deref()`) behind the scenes, effectively calling `print_length(&(*my_string))`.

Deref coercion allows for more ergonomic code by reducing the need for explicit dereferencing and enabling more seamless interaction between different types. It simplifies working with smart pointers and other types that provide dereference-like behavior through the `Deref` trait.