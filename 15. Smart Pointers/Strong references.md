# Strong references

Strong references

What are Strong references in rust?

___

In Rust, a strong reference is a type of reference that owns and controls the lifetime of the underlying value. It is the default form of ownership in Rust and is represented by the `Rc<T>` (Reference Counting) smart pointer.

When you create a strong reference to a value using `Rc<T>`, it increases the reference count by one, indicating that there is one more owner of the value. As long as there is at least one strong reference to the value, it will remain alive and accessible.

Strong references are useful in scenarios where you need to share ownership of a value among multiple parts of your program. With strong references, multiple parts of the program can have read and write access to the value and can freely clone and share the reference to it. When the last strong reference goes out of scope, the value will be deallocated automatically.

Here's an example that demonstrates the usage of strong references (`Rc<T>`) in Rust:

```rust
use std::rc::Rc;

struct Person {
    name: String,
    age: u32,
}

fn main() {
    // Create a shared Person instance
    let person = Rc::new(Person {
        name: String::from("Alice"),
        age: 25,
    });

    // Clone the reference to the Person
    let person1 = Rc::clone(&person);
    let person2 = Rc::clone(&person);

    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Reference count: {}", Rc::strong_count(&person));
}
```

In this example, we define a `Person` struct with a `name` field of type `String` and an `age` field of type `u32`. We create an instance of `Person` and wrap it in an `Rc<Person>` to create a shared reference. We then clone the reference twice using `Rc::clone` to create additional strong references.

We can access the fields of the `Person` struct using any of the strong references, and the reference count can be obtained using the `Rc::strong_count` function. When all the strong references go out of scope, the `Person` value will be deallocated automatically.

Strong references in Rust provide a form of shared ownership that allows multiple parts of the program to access and manipulate the same value. They ensure memory safety by enforcing borrowing rules and automatically handling deallocation when the value is no longer needed.