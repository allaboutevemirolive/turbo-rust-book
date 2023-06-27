# Weak references

What is Weak references in rust?

___

In Rust, a weak reference is a type of reference that does not contribute to the ownership of the underlying value. It is typically used in conjunction with `Rc` (Reference Counting) to create a form of reference that does not prevent the value from being deallocated.

Unlike a strong reference (`Rc`), a weak reference does not increment the reference count. It allows you to access the value if it still exists, but it does not prevent the value from being dropped and deallocated when there are no more strong references to it.

Weak references are useful in scenarios where you need to break potential reference cycles. A reference cycle occurs when two or more values hold strong references to each other, causing a situation where they can never be dropped because the reference counts never reach zero. By using weak references, you can break the cycle and allow the values to be deallocated when they are no longer needed.

In Rust, weak references are represented by the `Weak<T>` type, which is created from an `Rc<T>` using the `Rc::downgrade` method. Here's an example that demonstrates the usage of weak references:

```rust
use std::rc::{Rc, Weak};

struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

fn main() {
    let node1 = Rc::new(Node {
        value: 1,
        next: None,
    });

    let node2 = Rc::new(Node {
        value: 2,
        next: Some(Rc::downgrade(&node1)),
    });

    // Create a weak reference to `node2`
    let weak_ref = Rc::downgrade(&node2);

    // Access the value if it still exists
    if let Some(node) = weak_ref.upgrade() {
        println!("Value: {}", node.value);
    } else {
        println!("Value no longer exists");
    }

    // Dropping `node2`, no strong references left
    drop(node2);

    // Try to access the value again
    if let Some(node) = weak_ref.upgrade() {
        println!("Value: {}", node.value);
    } else {
        println!("Value no longer exists");
    }
}
```

In this example, we create a simple linked list using `Node` structs. Each `Node` holds a value of type `i32` and an optional `next` field, which is an `Option<Rc<Node>>` representing the next node in the list. To break the potential reference cycle, we use `Rc::downgrade` to create a weak reference (`Weak<Node>`) from `node2` to `node1`.

We then demonstrate accessing the value of the weak reference using the `upgrade` method. If the value still exists (i.e., there is at least one strong reference to it), we print its value. After dropping `node2`, which was the last strong reference, the value no longer exists, and attempting to upgrade the weak reference will indicate that.