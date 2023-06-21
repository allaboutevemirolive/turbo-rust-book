In Rust, consuming adaptors are methods or functions that consume the elements of an iterator, while non-consuming adaptors are methods or functions that produce new iterators without consuming the original one. Here's a list of some common consuming and non-consuming adaptors in Rust:

Consuming Adaptors:
- `collect`: Consumes the iterator and collects its elements into a new collection, such as a vector or hashmap.
- `count`: Consumes the iterator and counts the number of elements.
- `max`/`min`: Consumes the iterator and finds the maximum or minimum element, respectively.
- `sum`: Consumes the iterator and calculates the sum of the elements.
- `fold`: Consumes the iterator and applies a function to each element, accumulating a final result.

Non-consuming Adaptors:
- `map`: Transforms each element of the iterator into a new element using a provided function, producing a new iterator.
- `filter`: Filters the elements of the iterator based on a predicate, producing a new iterator with only the matching elements.
- `enumerate`: Wraps each element of the iterator with its index, producing a new iterator of tuples `(index, element)`.
- `skip`: Skips a specified number of elements from the iterator, producing a new iterator.
- `take`: Takes a specified number of elements from the iterator, producing a new iterator with only those elements.
- `peekable`: Wraps the iterator, allowing you to peek at the next element without consuming it.

These are just a few examples of commonly used consuming and non-consuming adaptors in Rust. Rust's standard library provides a rich set of iterator adaptors that offer various operations for transforming, filtering, and consuming elements in a flexible and efficient manner.

It's important to note that the distinction between consuming and non-consuming adaptors helps ensure code clarity, memory safety, and predictable behavior. Choosing the appropriate adaptor based on your requirements allows you to write concise and expressive code while maintaining ownership and avoiding potential bugs.