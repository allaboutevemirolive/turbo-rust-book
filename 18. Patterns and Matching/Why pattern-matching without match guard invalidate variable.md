# Why pattern-matching without match guard invalidate variable

Then how does using pattern-matching without match guard invalidate the use of the same variable after the pattern-matching block?

___


When you use the same variable name in a pattern match arm without a match guard, it shadows the outer variable with the same name. This means that a new binding is created within the match arm, and the outer variable becomes inaccessible within that scope.

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

In this code, the variable `value` is defined in the outer scope. However, within the match arm, a new variable binding with the same name `value` is introduced. This pattern shadowing creates a new variable that takes precedence within the match arm, effectively hiding the outer variable with the same name.

Since the inner `value` binding in the match arm shadows the outer `value` variable, the outer variable becomes inaccessible within the scope of the match arm. If you attempt to use `value` outside the match expression, it results in a compilation error because the outer variable has been shadowed and is no longer visible.

This behavior is intentional and is part of Rust's scoping rules. It helps prevent accidental confusion and provides clarity within the match arm. However, it can also lead to unintended consequences if you rely on the outer variable within the same scope.

To avoid this issue and have access to the outer variable after the pattern match, you can either rename the variable in the pattern match arm or use a different variable name to avoid shadowing. By choosing unique variable names, you can ensure that both the outer variable and the pattern match variable remain accessible within their respective scopes.