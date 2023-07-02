# Implementing an Unsafe Trait

We can use unsafe to implement an unsafe trait.

A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify.

We declare that a trait is `unsafe` by adding the `unsafe` keyword before `trait` and marking the implementation of the trait as `unsafe` too, as shown in Listing 19-11.

```rust
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
```

Listing 19-11: Defining and implementing an unsafe trait

By using `unsafe impl`, we’re promising that we’ll uphold the invariants that the compiler can’t verify.

As an example, recall the `Sync` and `Send` marker traits we discussed in the “Extensible Concurrency with the `Sync` and `Send` Traits” section in Chapter 16: the compiler implements these traits automatically if our types are composed entirely of `Send` and `Sync` types.

If we implement a type that contains a type that is not `Send` or `Sync`, such as raw pointers, and we want to mark that type as `Send` or `Sync`, we must use `unsafe`.

Rust can’t verify that our type upholds the guarantees that it can be safely sent across threads or accessed from multiple threads; therefore, we need to do those checks manually and indicate as such with `unsafe`.

[[Accessing Fields of a Union]]
