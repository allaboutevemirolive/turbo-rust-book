The statement you provided suggests an additional experiment to understand Rust's borrowing rules and the behavior of closures:

Between the closure definition and the closure call, the code suggests adding a `println!` statement that attempts to print the content of `list` using an immutable borrow. 

However, this is not allowed because Rust's borrowing rules prevent having both a [[mutable borrow]] and an [[immutable borrow]] of the same variable in the same scope. 

Since the closure captures `list` mutably, an immutable borrow is not allowed until the mutable borrow is released.

If you attempt to add a `println!` statement to print the contents of `list` between the closure definition and the closure call, you will encounter a compilation error indicating a violation of the borrowing rules.

To overcome this limitation and allow the closure to take ownership of the values it uses in the environment (even though the closure body doesn't strictly need ownership), you can use the `move` keyword before the parameter list of the closure. 

This signals that the closure should take ownership of its captured variables rather than borrowing them. 

However, in the example you provided, the closure modifies the vector, so using `move` in this case would not be appropriate.

In summary, the provided code demonstrates how a mutable closure can capture a mutable reference to a variable from its surrounding scope and modify its contents. 

The borrowing rules of Rust prevent simultaneous [mutable] and [immutable borrows] of the same variable in the same scope, which is why attempting an immutable borrow between the closure definition and the closure call would result in a compilation error.