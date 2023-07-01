# Does match guard nullify the variable binding

Why will using match guard in an arm nullify the variable binding? 

Explain to me in detail how this is not considered as binding

___


I apologize for any confusion caused by my previous statement. Let me clarify and correct that statement.

When using a match guard in a pattern match arm, it does `not nullify or invalidate the variable binding`. The variable binding established in the pattern still holds and remains in scope within the match arm. The match guard expression is evaluated after the pattern match succeeds, and it serves as an additional condition that must be satisfied for the match arm to be executed.

To clarify, a variable binding in a pattern match arm is a form of binding. It introduces a new variable name and associates it with a specific value based on the pattern match. The variable binding remains in effect within the match arm, allowing you to use the matched value under that variable name.

Let's consider an example to demonstrate this behavior:

```rust
fn main() {
    let value = Some(5);

    match value {
        Some(value) if value > 0 => {
            println!("Value in match arm: {}", value);
        }
        Some(_) => {
            println!("Value is zero or negative");
        }
        None => {}
    }
}
```

In this code, the `Some(value)` pattern match arm introduces a new variable binding with the name `value`. The variable `value` is bound to the inner value of the `Some` variant if it is greater than zero.

The match guard `if value > 0` is an additional condition that must be true for the match arm to be executed. It does not nullify or invalidate the variable binding. Instead, it adds an extra constraint that filters which values are handled by that match arm. The variable `value` is still accessible and holds its value within the match arm, allowing you to use it for further computations or print statements.

To summarize, using a match guard in a pattern match arm does not nullify or invalidate the variable binding. The variable binding remains intact within the match arm, and the match guard acts as an additional condition that must be satisfied for the match arm to be executed. The match guard helps filter which values are handled by the match arm based on the specified condition, while the variable binding itself is still in effect.