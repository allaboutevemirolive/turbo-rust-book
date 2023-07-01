# What is "match guard conditional" in rust

What is "match guard conditional" in rust?

___

In Rust, a "match guard conditional" refers to a condition that can be added to a specific arm of a `match` expression. It allows you to further refine the pattern matching by specifying an additional condition that must be satisfied for that arm to be executed.

Here's an example to illustrate the concept:

```rust
fn main() {
    let number = Some(7);

    match number {
        Some(x) if x < 5 => println!("Less than 5"),
        Some(x) if x > 5 => println!("Greater than 5"),
        Some(_) => println!("Equal to 5"),
        None => println!("None"),
    }
}
```

In this example, the `match` expression is used to handle different cases based on the value of `number`, which is of type `Option<i32>`. We have multiple arms within the `match` block, each representing a different pattern and behavior.

The key part related to match guard conditionals is `if x < 5` and `if x > 5` within the `Some` arms. These conditions act as additional filters to narrow down the matching criteria. If the pattern matches the `Some` variant, the associated guard condition is evaluated. If the condition evaluates to `true`, the arm is executed. Otherwise, the program proceeds to the next arm in the `match` expression.

In this specific example, if `number` is `Some(x)` and `x` is less than 5, the first arm is executed, printing "Less than 5". If `x` is greater than 5, the second arm is executed, printing "Greater than 5". If `x` is neither less nor greater than 5, it must be equal to 5, and the third arm is executed, printing "Equal to 5". If `number` is `None`, the last arm is executed, printing "None".