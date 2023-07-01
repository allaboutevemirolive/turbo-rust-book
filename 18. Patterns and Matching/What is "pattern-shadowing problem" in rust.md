# What is "pattern-shadowing problem" in rust

The "pattern-shadowing problem" refers to a potential issue that can arise in Rust when using pattern matching. It occurs when a variable name used in a pattern match arm shadows an existing variable in the outer scope.

Consider the following example:

```rust
fn main() {
    let value = Some(5);

    match value {
        Some(value) => {
            println!("Value in match arm: {}", value);
        }
        None => {}
    }

    // Error: Cannot use `value` here because it has been shadowed
    // println!("Value outside match: {}", value);
}
```

In this code, we have a variable named `value` in the outer scope, and we also have a pattern match arm that uses the same variable name `value`. The variable `value` in the match arm shadows the outer `value` variable, meaning it creates a new binding that takes precedence within that specific arm.

The problem arises when we try to use the `value` variable outside the match expression. Since the pattern match arm has its own `value` binding, the outer `value` variable is effectively "shadowed" and cannot be accessed in the outer scope. This results in a compilation error if we try to use `value` outside the match expression.

To avoid the pattern-shadowing problem, you can use a different variable name in the match arm or rename the variable in the outer scope:

```rust
fn main() {
    let value = Some(5);

    match value {
        Some(inner_value) => {
            println!("Value in match arm: {}", inner_value);
        }
        None => {}
    }

    // No error: `value` is accessible here
    println!("Value outside match: {:?}", value);
}
```

In this modified code, the variable in the match arm is renamed to `inner_value`, which avoids shadowing the outer `value` variable. As a result, we can use `value` both within and outside the match expression without encountering any issues.

To summarize, the pattern-shadowing problem occurs when a variable name used in a pattern match arm shadows an existing variable in the outer scope, making the outer variable inaccessible within the affected scope. It is generally advisable to use different variable names or rename variables to avoid this problem and ensure clarity and correctness in your code.