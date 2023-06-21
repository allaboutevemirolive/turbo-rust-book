What is "consuming adaptors" in the statement below? 

"Methods that call `next` are called consuming adaptors because calling them uses up the iterator."

___

In Rust, consuming adaptors are methods or functions that consume (use up) the elements of an iterator by repeatedly calling the `next` method on the iterator until it returns `None`. These adaptors are called "consuming" because they consume and exhaust the iterator, leaving it in a state where no more elements can be produced.

The statement you provided is highlighting the fact that methods which call `next` on an iterator are considered consuming adaptors. These methods typically perform some action on each element of the iterator or transform the elements in some way. By repeatedly calling `next` on the iterator, these methods consume (take ownership of) the elements one by one until the iterator is depleted.

Here's an example that demonstrates the usage of a consuming adaptor, specifically the `sum` method, which calculates the sum of all elements in an iterator:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.into_iter().sum();

    println!("Sum: {}", sum);
}
```

In this example, `into_iter` is used to create a consuming iterator from the vector `numbers`. Then, the `sum` method is called on the iterator, which consumes the elements and calculates their sum. After the `sum` method is called, the iterator is exhausted, and its elements are used up.

Other examples of consuming adaptors in Rust's standard library include `collect`, `fold`, `for_each`, and `min`/`max`. These adaptors allow you to perform various operations on the elements of an iterator by consuming them in the process.

It's important to note that once an iterator has been consumed by a consuming adaptor, it cannot be reused or iterated over again. If you need to perform multiple operations on the same iterator, you may need to recreate the iterator or choose non-consuming adaptors that produce new iterators without consuming the original one.

Consuming adaptors are a powerful feature of Rust's iterator system that enable concise and expressive data processing pipelines while maintaining ownership and memory safety.


References:

- [[Why iterator has been consumed by consuming adaptor, it cannot be reused again]]
- [[List of consuming adaptors and non-consuming adaptors in Rust]]