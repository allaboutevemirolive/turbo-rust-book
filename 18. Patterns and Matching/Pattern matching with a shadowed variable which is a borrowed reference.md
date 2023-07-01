# Pattern matching with a shadowed variable which is a borrowed reference

Show me a pattern matching using a shadowed variable, which is a borrowed reference. 

I want to understand the implications of ownership and borrowing.

___


Certainly! Here's an example that demonstrates pattern matching with a shadowed variable, where the shadowed variable is a borrowed reference:

```rust
fn main() {
    let value = Some(5);
    let reference = &value;

    match reference {
        &Some(ref value) => {
            println!("Value in match arm: {}", value);
        }
        &None => {}
    }

    // Error: Cannot borrow `value` as mutable because it is borrowed as immutable
    // value = Some(10);
}
```

In this code, we have a variable `value` that holds a `Some` variant, and we create a borrowed reference `reference` to `value`. The reference `reference` borrows the value immutably.

In the pattern match arm, we use the `&Some(ref value)` pattern to match against the reference `reference`. The pattern creates a new variable binding `value` that shadows the outer `value` variable. The `ref` keyword is used to create a reference to the value within the `Some` variant without moving or copying it.

Within the match arm, the variable `value` is accessible and holds the borrowed value. You can perform operations or print statements using the shadowed `value` variable.

However, one important implication is that the outer variable `value` is no longer accessible within the match arm. This is because the borrowed reference `reference` is used in the pattern match, and it takes precedence over the outer `value`. The outer `value` is effectively "frozen" for the duration of the match arm, ensuring that the immutable borrow of `reference` remains valid.

If you try to modify the outer `value` after the pattern match arm, it will result in a compilation error because the reference `reference` is still in scope and holding an immutable borrow of `value`.

In summary, using a shadowed variable in a pattern match, especially when it involves borrowed references, affects the ownership and borrowing rules in Rust. The shadowed variable becomes the active binding within the match arm, while the outer variable remains inaccessible within that scope to preserve the borrow guarantees. This behavior ensures that Rust maintains its safety guarantees related to ownership and borrowing.