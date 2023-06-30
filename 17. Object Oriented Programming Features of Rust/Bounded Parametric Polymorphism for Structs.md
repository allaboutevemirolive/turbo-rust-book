# Bounded Parametric Polymorphism for Structs

Bounded parametric polymorphism with structs:

You can use generics and trait bounds with structs to create generic data structures that can hold values of different types while enforcing certain constraints. By defining a struct with a generic type parameter and applying trait bounds to that parameter, you can ensure that only types that satisfy the specified traits can be used with the struct.

Here's an example of a generic struct Container with a type parameter T that must implement the Clone trait:

```rust
struct Container<T: Clone> {
    value: T,
}

fn main() {
    let string_container = Container { value: "Hello".to_string() };
    let int_container = Container { value: 42 };
}
```

In this example, the Container struct can hold values of any type T that implements the Clone trait. This constraint is expressed through the trait bound T: Clone in the struct definition. You can then create instances of the Container struct with different types as long as they satisfy the Clone trait.