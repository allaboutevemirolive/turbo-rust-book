Explain to me what does it mean by this statment? 

"since we are iterating the elements by value, the elements are moved and so it is impossible to print its elements after the for loop."

___

The statement you mentioned is referring to the behavior of iterating over elements by value in Rust. Let's break it down to understand its meaning:

In Rust, when you iterate over elements by value, you consume (move) those elements from the original collection. This means that ownership of each element is transferred to the loop body, and the original collection no longer has access to those elements.

In the context of the statement, it is saying that because the elements are moved during the iteration, you cannot access or print them after the `for` loop. This is because the ownership of the elements has been transferred to the loop body, and the original collection is left without those elements.

Here's an example to illustrate this behavior:

```rust
fn main() {
    let vector = vec![1, 2, 3, 4, 5];

    for element in vector {
        println!("{}", element);
    }

    // Error: `vector` is no longer valid, as its elements were moved
    println!("{:?}", vector);
}
```

In this example, the `for` loop iterates over the elements of the `vector` by value. Each element is moved into the loop's variable `element`, allowing you to print it. However, after the loop, if you try to access `vector`, you'll get a compilation error. This is because the elements were moved and are no longer accessible within `vector`.

To overcome this limitation and still have access to the elements after the loop, you can either iterate over references to the elements (`&element`) or use the `into_iter` method, which consumes the collection but allows you to iterate over the owned elements.

Remember, when iterating by value, the original collection is modified during the iteration, and you can't access its elements afterwards without making changes to your code.