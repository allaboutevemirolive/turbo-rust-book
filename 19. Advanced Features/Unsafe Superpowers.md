# Unsafe Superpowers

To switch to unsafe Rust, use the `unsafe` keyword and then start a new block that holds the unsafe code.

You can take five actions in unsafe Rust that you can’t in safe Rust, which we call *unsafe superpowers*.

Those superpowers include the ability to:

- Dereference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
- Access fields of unions

It’s important to understand that `unsafe` doesn’t turn off the borrow checker or disable any other of Rust’s safety checks: if you use a reference in `unsafe` code, it will still be checked.

The `unsafe` keyword only gives you access to these five features that are then not checked by the compiler for memory safety.

You’ll still get some degree of safety inside of an unsafe block.



In addition, `unsafe` does not mean the code inside the block is necessarily dangerous or that it will definitely have memory safety problems: the intent is that as the programmer, you’ll ensure the code inside an `unsafe` block will access memory in a valid way.



People are fallible, and mistakes will happen, but by requiring these five `unsafe` operations to be inside blocks annotated with `unsafe` you’ll know that any errors related to memory safety must be within an `unsafe` block.

Keep `unsafe` blocks small; you’ll be thankful later when you investigate memory bugs.



To isolate unsafe code as much as possible, it’s best to enclose unsafe code within a safe abstraction and provide a safe API, which we’ll discuss later in the chapter when we examine unsafe functions and methods.

Parts of the standard library are implemented as safe abstractions over unsafe code that has been audited.

Wrapping unsafe code in a safe abstraction prevents uses of `unsafe` from leaking out into all the places that you or your users might want to use the functionality implemented with `unsafe` code, because using a safe abstraction is safe.



Let’s look at each of the five unsafe superpowers in turn.

We’ll also look at some abstractions that provide a safe interface to unsafe code.



[[Dereferencing a Raw Pointer]]
