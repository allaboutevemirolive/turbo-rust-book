# public API

What is a public API in Rust? I have seen this term use many times in rust documentation

___

In Rust, a public API refers to the interface or set of functions, types, and traits that are intended to be used and accessed by users of a Rust library or crate. It defines the publicly accessible parts of the library that can be used to interact with its functionality.

When creating a Rust library, it is common practice to define which parts of the code are intended to be part of the public API. These are typically specified using the `pub` keyword in Rust. By marking certain items as public, you make them accessible to other code outside the library, allowing users to use those items in their own programs.

By defining a clear and well-designed public API, library authors can provide a clean and intuitive interface for other developers to interact with their code. This helps in promoting code reuse, maintainability, and collaboration within the Rust ecosystem.

It's worth noting that not everything in a Rust library needs to be part of the public API. The `pub` keyword is used selectively to expose only the necessary parts, while keeping implementation details and internal functions hidden from external users. This encapsulation provides better control over the library's internals and allows for easier maintenance and evolution of the codebase.

When you come across the term "public API" in Rust documentation, it generally refers to the set of items that are marked as `pub` and meant to be used by external code. These items form the contract between the library and its users, defining how they can interact with the library and what functionality is available to them.