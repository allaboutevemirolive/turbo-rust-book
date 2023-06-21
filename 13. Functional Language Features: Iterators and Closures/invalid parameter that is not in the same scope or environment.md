So the code below is invalid since paramete is not in the same scope with closure, right? 

```rust
{
	let x = 5;
}

let closure = || { println!("Captured variable: {}", x); };
```

___

Yes, you are correct. The code you provided is invalid because the variable `x` is not in the same scope as the closure. In Rust, closures can only capture variables that are in scope at the point of their definition. Once a variable goes out of scope, it is no longer accessible to the closure.

In your code example, the variable `x` is declared within a block scope delimited by curly braces `{ }`. This means that `x` is only valid and accessible within that block. Outside of the block, `x` is out of scope, and the closure defined afterwards cannot capture it.

To make the code valid and allow the closure to capture `x`, you need to define `x` in a scope that encompasses both the variable and the closure, like this:

```rust
let x = 5;

let closure = || {
    println!("Captured variable: {}", x);
};
```

In this modified code, the variable `x` is defined in a scope that is accessible to both the variable declaration and the closure. Now, the closure can capture `x` from its surrounding environment and access its value when executed.

Remember, closures can only capture variables from the same or outer scopes where they are defined. They cannot capture variables from inner or nested scopes.