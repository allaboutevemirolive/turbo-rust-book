# Summary

This isn’t the last you’ll see of concurrency in this book: the project in Chapter 20 will use the concepts in this chapter in a more realistic situation than the smaller examples discussed here.


As mentioned earlier, because very little of how Rust handles concurrency is part of the language, many concurrency solutions are implemented as crates.

These evolve more quickly than the standard library, so be sure to search online for the current, state-of-the-art crates to use in multithreaded situations.


The Rust standard library provides channels for message passing and smart pointer types, such as `Mutex<T>` and `Arc<T>`, that are safe to use in concurrent contexts.

The type system and the borrow checker ensure that the code using these solutions won’t end up with data races or invalid references.

Once you get your code to compile, you can rest assured that it will happily run on multiple threads without the kinds of hard-to-track-down bugs common in other languages.

Concurrent programming is no longer a concept to be afraid of: go forth and make your programs concurrent, fearlessly!

Next, we’ll talk about idiomatic ways to model problems and structure solutions as your Rust programs get bigger.

In addition, we’ll discuss how Rust’s idioms relate to those you might be familiar with from object-oriented programming.

