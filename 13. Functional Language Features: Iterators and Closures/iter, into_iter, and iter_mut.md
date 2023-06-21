Generate rust's main function that implements iter, into_iter, and iter_mut and explain the difference between them

___

Certainly! Here's an example of a Rust `main` function that demonstrates the usage of `iter`, `into_iter`, and `iter_mut`:

```rust
fn main() {
    let vector = vec![1, 2, 3, 4, 5];

    // Using `iter` to create an immutable iterator
    for item in vector.iter() {
        println!("Immutable: {}", item);
    }

    // Using `into_iter` to consume the vector and create a consuming iterator
    for item in vector.into_iter() {
        println!("Consuming: {}", item);
    }

    // Using `iter_mut` to create a mutable iterator
    let mut mutable_vector = vec![1, 2, 3, 4, 5];
    for item in mutable_vector.iter_mut() {
        *item *= 2; // Doubling each element
    }
    println!("Mutable: {:?}", mutable_vector);
}
```

Explanation:
- `iter`: The `iter` method returns an iterator over immutable references to the elements of a collection. It allows you to iterate over the collection without modifying its elements. The original collection remains accessible and unchanged. In the example, `vector.iter()` returns an iterator, and each element is printed using a for loop.

- `into_iter`: The `into_iter` method consumes the collection and returns an iterator that takes ownership of the elements. It allows you to consume the collection, which means the original collection is no longer usable after the iteration. In the example, `vector.into_iter()` returns an iterator, and each element is printed using a for loop. Note that after this iteration, `vector` cannot be used anymore.

- `iter_mut`: The `iter_mut` method returns an iterator over mutable references to the elements of a collection. It allows you to modify the elements while iterating. The original collection remains accessible but can be modified through the mutable references obtained from the iterator. In the example, `mutable_vector.iter_mut()` returns an iterator, and each element is doubled using a for loop. The changes are reflected in the `mutable_vector` after the iteration.

In summary, `iter` provides immutable references to the elements, `into_iter` consumes the collection and provides owned elements, and `iter_mut` provides mutable references to the elements, allowing modification of the original collection.