# Dynamic dispatch

Explain to me in detail about "Dynamic dispatch" in rust and give example

___

Dynamic dispatch in Rust refers to the ability to choose a specific implementation of a trait (interface) at runtime based on the actual type of the object being referenced. It allows you to write more generic code that can work with different types without knowing the concrete type at compile-time.

In Rust, dynamic dispatch is achieved using trait objects, which are created by using the `dyn` keyword. To use dynamic dispatch, you define a trait and then implement it for various types. You can then create a reference or a box to the trait object, which can hold any object implementing that trait.

Here's an example to illustrate dynamic dispatch in Rust:

```rust
trait Animal {
    fn sound(&self);
}

struct Cat;
struct Dog;

impl Animal for Cat {
    fn sound(&self) {
        println!("Meow!");
    }
}

impl Animal for Dog {
    fn sound(&self) {
        println!("Woof!");
    }
}

fn make_sound(animal: &dyn Animal) {
    animal.sound();
}

fn main() {
    let cat = Cat;
    let dog = Dog;

    make_sound(&cat); // Dynamic dispatch: Cat's sound method is called.
    make_sound(&dog); // Dynamic dispatch: Dog's sound method is called.
}
```

In this example, we define the `Animal` trait with a single method `sound()`. We then implement this trait for `Cat` and `Dog` structs. The `make_sound` function takes a reference to a trait object of type `dyn Animal`, which means it can accept any object that implements the `Animal` trait.

Inside the `main` function, we create instances of `Cat` and `Dog`, and then we call the `make_sound` function with references to these objects. The Rust compiler performs dynamic dispatch to choose the appropriate `sound` method based on the actual type of the object being referenced.

When `make_sound(&cat)` is called, the `sound` method for `Cat` is invoked and "Meow!" is printed. When `make_sound(&dog)` is called, the `sound` method for `Dog` is invoked and "Woof!" is printed. The specific implementation of the `sound` method is determined at runtime based on the actual type of the object being referenced.

Dynamic dispatch allows you to write code that can work with different types implementing the same trait without having to know the concrete types at compile-time, providing flexibility and enabling polymorphic behavior in Rust.