# 2_Ownership in pattern-matching block

1. Does the pattern-matching block also enforce ownership? 

Meaning the tuple moves into the pattern-matching block. 

2. After pattern-matching block, can I print the tuple outside the block again?

___


In Rust, pattern matching can enforce ownership if the matched value is moved into the pattern-matching arms. However, in the case of your provided code example, the ownership of the tuple is not transferred into the pattern-matching block. Therefore, you can still access and use the tuple after the match expression.

Here's your code example with an additional print statement after the match expression:

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    println!("Tuple: {:?}", numbers);
}
```

In this modified example, after the match expression, we added a print statement to display the `numbers` tuple. Here, the tuple is printed outside the match expression.

Since the ownership of the `numbers` tuple is not moved into the match arms, you can still access and print the tuple outside the match expression without any issues. The output will be:

```
Some numbers: 2, 32
Tuple: (2, 4, 8, 16, 32)
```

As you can see, the tuple is printed both inside and outside the match expression, indicating that the tuple remains accessible after the pattern matching block.