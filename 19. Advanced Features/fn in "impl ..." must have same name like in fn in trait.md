# fn in "impl ..." must have same name like in fn in trait

I realize that impl Animal needs to have function name same as in trait otherwise it will not work.

Why rust do something like this? What is the benefit?

```rust
trait Animal {
    // Compile time error!
    // fn baby_name() -> String;
}

struct Dog;

impl Animal for Dog {
    // not a member of trait `Animal`
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
	// ...::baby_name() => not found in `Animal`
	println!("Sometimes dog also called {}", <Dog as Animal>::baby_name() );
}
```

Output:

```rust
   Compiling playground v0.0.1 (/playground)
error[E0407]: method `baby_name` is not a member of trait `Animal`
  --> src/main.rs:14:5
   |
14 | /     fn baby_name() -> String {
15 | |         String::from("puppy")
16 | |     }
   | |_____^ not a member of trait `Animal`

error[E0576]: cannot find method or associated constant `baby_name` in trait `Animal`
  --> src/main.rs:22:60
   |
22 |     println!("Sometimes dog also called {}", <Dog as Animal>::baby_name() );
   |                                                               ^^^^^^^^^ not found in `Animal`

Some errors have detailed explanations: E0407, E0576.
For more information about an error, try `rustc --explain E0407`.
error: could not compile `playground` (bin "playground") due to 2 previous errors
```
___


In Rust, when implementing a trait for a type, the names of the functions defined in the trait and their corresponding implementations in the type must match. This requirement ensures that the trait contract is properly fulfilled by the implementing type. If the names don't match, the compiler will consider them as unrelated functions, which can lead to unexpected behavior or errors.

The main benefit of this requirement is that it promotes `clarity`, `consistency`, and `reliability`. By enforcing matching function names, Rust ensures that the behavior specified in the trait is consistently implemented across different types. This allows users of the trait to have clear expectations about the functionality provided by different implementations.

Additionally, this requirement enables dynamic dispatch, where trait objects can be used to refer to different concrete types that implement the trait. Since trait objects are based on vtables (virtual tables), which store function pointers, the function names in the trait and their implementations must match for the vtable to be correctly populated and for dynamic dispatch to work as expected.

In your example, if you uncomment the implementation of `baby_name()` for the `Dog` type, but change its function name to something other than `baby_name()`, it will no longer fulfill the requirements of the `Animal` trait. As a result, you will encounter a compilation error when attempting to implement the trait for `Dog`.

By enforcing matching function names, Rust ensures that the implementation of a trait is consistent and reliable across different types, promoting safer and more predictable code. It's a design choice that helps maintain the integrity of the trait system and improves the overall robustness of Rust programs.