What is private interfaces in Rust?

In Rust, private interfaces refer to the parts of a module or struct that are not accessible or visible outside of the module they are defined in. Private interfaces are used to encapsulate implementation details and provide information hiding, ensuring that only the necessary and intended functionality is exposed to external code.

By default, everything in Rust is private, meaning it is only accessible within the same module. To define private interfaces, you can use the `pub` keyword to mark certain items (such as functions, structs, enums, or modules) as public, while leaving others as private.

Here's an example:

```rust
mod my_module {
    pub struct PublicStruct {
        pub field: i32,
        // pub private_field: f32,
        private_field: f32,
    }

    impl PublicStruct {
        pub fn public_method(&self) {
            // Accessing both public and private fields is allowed
            println!("Public field: {}", self.field);
            println!("Private field: {}", self.private_field);
        }

        fn private_method(&self) {
            // Only accessible within the module
            println!("Private method");
        }
    }

    pub fn public_function() {
        println!("Public function");
    }

    fn private_function() {
        println!("Private function");
    }
}

fn main() {
    let my_struct = my_module::PublicStruct {
        field: 42,
        private_field: 3.14,
    };

    my_struct.public_method();
    // Accessing private fields or invoking private methods is not allowed outside the module
    // my_struct.private_field; // This would be a compile-time error
    // my_struct.private_method(); // This would be a compile-time error

    my_module::public_function();
    // Invoking private functions is not allowed outside the module
    // my_module::private_function(); // This would be a compile-time error
}
```

In the above example, the `PublicStruct` has a `pub` field `field` that is accessible from outside the module, while `private_field` is not accessible outside the module. Similarly, the `public_method` is marked as `pub` and can be called from outside, while `private_method` is only accessible within the module.

The `public_function` is marked as `pub` and can be called from outside, whereas `private_function` is only callable within the module.

Using private interfaces allows you to hide implementation details and expose only the necessary parts of your code to maintain encapsulation, minimize dependencies, and provide a clear and well-defined public API. It helps in controlling access and maintaining a stable interface while allowing internal modifications without affecting external code.