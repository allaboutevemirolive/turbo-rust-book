# 2_difference between macros and functions in rust

## Difference between Macros and Functions in Rust

In Rust, functions and macros are both used to organize code and make it more reusable. However, there are some key differences between the two.

### Functions
Functions are a set of statements that perform a specific task. They are defined using the `fn` keyword followed by the function name and a set of parentheses. The curly brackets tell the compiler where the function body begins and ends[1]. Functions are prevalent in Rust code and are used for code organization, encapsulation, and code reusability[2]. 

Functions are executed at runtime and can take arguments and return a value. Rust functions return exactly one value, and you declare the type after an "arrow," which is a dash followed by a greater-than sign[3]. 

### Macros
Macros are a way of writing code that writes other code, which is known as metaprogramming. They are used to generate code based on input, simplify repetitive patterns, and make code more concise[4]. Rust macros are executed at compile time and are expanded into abstract syntax trees, rather than string preprocessing, so you don't get unexpected precedence bugs[5]. 

Macros are created using the `macro_rules!` macro. They can be used to define custom attributes, generate code based on input, and more[6]. Rust macros are hygienic, meaning that identifiers from two different contexts cannot collide[7]. 

### Differences
One of the main differences between macros and functions is that macros are executed at compile time, while functions are executed at runtime[5]. Macros are also more complex to define than functions because you're writing Rust code that writes Rust code[6]. 

Another difference is that Rust macros are significantly different from C macros. Rust macros are expanded into abstract syntax trees, rather than string preprocessing, so you don't get unexpected precedence bugs[8]. 

Macros are generally used in situations where functions fail to provide the desired solution, where you have code that is quite repetitive, or where you need to generate code based on input[9]. 

In summary, functions and macros are both important constructs in Rust. Functions are used for code organization, encapsulation, and code reusability, while macros are used for metaprogramming and generating code based on input.

Citations:
[1] https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
[2] https://www.makeuseof.com/rust-functions-understanding/
[3] https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/functions.html
[4] https://www.programiz.com/rust/macro
[5] https://doc.rust-lang.org/rust-by-example/macros.html
[6] https://doc.rust-lang.org/book/ch19-06-macros.html
[7] https://danielkeep.github.io/quick-intro-to-macros.html
[8] https://stackoverflow.com/questions/29871967/what-is-the-difference-between-macros-and-functions-in-rust
[9] https://subscription.packtpub.com/book/application-development/9781838828103/9/ch09lvl1sec76/when-to-use-and-not-use-rust-macros