# Function-like macros

Function-like macros define macros that look like function calls.

Similarly to `macro_rules!` macros, they’re more flexible than functions; for example, they can take an unknown number of arguments.

However, `macro_rules!` macros can be defined only using the match-like syntax we discussed in the section “Declarative Macros with macro_rules! for General Metaprogramming” earlier.

Function-like macros take a `TokenStream` parameter and their definition manipulates that `TokenStream` using Rust code as the other two types of procedural macros do.

An example of a function-like macro is an `sql!` macro that might be called like so:

```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

This macro would parse the SQL statement inside it and check that it’s syntactically correct, which is much more complex processing than a `macro_rules!` macro can do.

The `sql!` macro would be defined like this:

```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

This definition is similar to the custom derive macro’s signature: we receive the tokens that are inside the parentheses and return the code we wanted to generate.

[[Summary]]
Whew! Now you have some Rust features in your toolbox that you likely won’t use often, but you’ll know they’re available in very particular circumstances.We’ve introduced several complex topics so that when you encounter them in error message suggestions or in other peoples’ code, you’ll be able to recognize these concepts and syntax.Use this chapter as a reference to guide you to solutions.

