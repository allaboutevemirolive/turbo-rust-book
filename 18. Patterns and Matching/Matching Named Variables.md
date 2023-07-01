# Matching Named Variables

Named variables are irrefutable patterns that match any value, and we’ve used them many times in the book.

However, there is a complication when you use named variables in `match` expressions.

Because `match` starts a new scope, variables declared as part of a pattern inside the `match` expression will shadow those with the same name outside the `match` construct, as is the case with all variables.

In Listing 18-11, we declare a variable named `x` with the value `Some(5)` and a variable `y` with the value `10`.

We then create a match expression on the value `x`.

Look at the patterns in the match arms and `println!` at the end, and try to figure out what the code will print before running this code or reading further.


QA:

[[What is "Some"]]

[[Code implementation for "Some"]]

Filename: src/main.rs

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}
```

output:

```rust
Matched, y = 5
at the end: x = Some(5), y = 10
```

Listing 18-11: A `match` expression with an arm that introduces a shadowed variable `y`

Let’s walk through what happens when the `match` expression runs.

The pattern in the first match arm doesn’t match the defined value of `x`, so the code continues.



The pattern in the second match arm introduces a new variable named `y` that will match any value inside a `Some` value.

Because we’re in a new scope inside the match expression, this is a new `y` variable, not the y we declared at the beginning with the value 10.

This new `y` binding will match any value inside a `Some`, which is what we have in `x`.

Therefore, this new `y` binds to the inner value of the `Some` in `x`.

That value is `5`, so the expression for that arm executes and prints `Matched, y = 5`.



If `x` had been a `None` value instead of `Some(5)`, the patterns in the first two arms wouldn’t have matched, so the value would have matched to the underscore.

We didn’t introduce the `x` variable in the pattern of the underscore arm, so the `x` in the expression is still the outer `x` that hasn’t been shadowed.

In this hypothetical case, the `match` would print `Default case, x = None`.



When the `match` expression is done, its scope ends, and so does the scope of the inner `y`.

The last `println!` produces `at the end: x = Some(5), y = 10`.



To create a `match` expression that compares the values of the outer x and y, rather than introducing a shadowed variable, we would need to use a match guard conditional instead.

We’ll talk about match guards later in the “Extra Conditionals with Match Guards” section.

QA:

[[What is "match guard conditional" in rust]]



[[Multiple Patterns]]
