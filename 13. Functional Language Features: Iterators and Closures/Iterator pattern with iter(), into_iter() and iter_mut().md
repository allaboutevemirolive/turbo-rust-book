[link](https://www.geekabyte.io/2022/08/rust-iterator-pattern-with-iter.html)

### Rust Iterator pattern with iter(), into_iter() and iter_mut() methods

Let's create a _vec_ of integers, iterate through and print the individual values, and then afterward, print out the whole _vec_. Here is the code:

```rust
fn main() {
   let some_ints = vec![1,2,3,4,5];
   for i in some_ints {
       dbg!(i);
   }

   dbg!(some_ints);
}
```

Simple right? Well nope. Trying to compile the above code will fail with the following errors:

```
error[E0382]: use of moved value: `some_ints`
   --> src/main.rs:9:9
    |
4   |    let some_ints = vec![1,2,3,4,5];
    |        --------- move occurs because `some_ints` has type `Vec<i32>`, 
    which does not implement the `Copy` trait
5   |    for i in some_ints {
    |             --------- `some_ints` moved due to this implicit call to `.into_iter()`
...
9   |    dbg!(some_ints);
    |         ^^^^^^^^^ value used here after move
    |
note: this function takes ownership of the receiver `self`, which moves `some_ints`
help: consider iterating over a slice of the `Vec<i32>`'s 
content to avoid moving into the `for` loop
    |
5   |    for i in &some_ints {
    |             +

For more information about this error, try `rustc --explain E0382`
```

Why is this the case? Why is the borrow checker preventing the use of a _vec_ after a simple iteration?

Well, the answer to that question lies in Rust's implementation of the Iterator pattern - which by the way, is what makes it possible to use the `for…in` syntax.

Iterators are not special or unique to Rust. The concept can be found in a handful of languages. I wrote about the Iterator pattern as it exists in JavaScript in the post [Iterables and Iterators in JavaScript](https://www.geekabyte.io/2019/06/iterables-and-iterators-in-javascript.html).

The unique thing about the Iterator pattern in Rust is its interaction with the borrow checker.

If this interaction with the borrow checker is not taken into consideration then it is possible to bump into certain confusing compile errors while attempting to use the iterator pattern.

So to get started answering the question of why the borrow checker prevents what looks like a legit code above, let's take a look at the Iterator pattern and what is special about it in Rust.

### Rust and Iterator Pattern.

An Iterator is a data structure that allows retrieval of elements from another data structure in sequence. In rust, it is any data structure that implements the `Iterator` trait.

```rust
#![allow(unused)]
fn main() {
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
 }
}
```

The `Iterator` trait defines a `next` method that when called, returns an `Option` of type `Self::Item`, which ends up being whatever the type of elements in what is being iterated over. Once the iteration reaches the end, a `None` is returned. For example:

```rust
#[test]
fn iterator_demo() {
 let v1 = vec![1, 2, 3];
 let mut v1_iter = v1.iter();

 assert_eq!(v1_iter.next(), Some(&1));
 assert_eq!(v1_iter.next(), Some(&2));
 assert_eq!(v1_iter.next(), Some(&3));
 assert_eq!(v1_iter.next(), None);

}
```

The above is simple enough. The part that interacts with the borrow checker is how the `next` method treats the value it is getting from the underlying structure it is iterating over.

It is possible that the `next` method of the `iterator` trait be implemented in such a way that it borrows the value immutable, borrows the value mutable, or even takes ownership of the values being iterated.

These three behaviors define 3 different distinct types of iterators.

- An iterator that borrows the value being iterated immutably
- An iterator that borrows the value being iterated mutably.
- An iterator that takes ownership of values being iterated

To demonstrate these 3 iterator types, we can take a _vec_ as usual and create these three distinct iterators and observe their behavior.

**An iterator that borrows the value being iterated immutably.**

```javascript
#[test]
fn iter_demo() {
    let v1 = vec![1, 2, 3];
    // the .iter() method creates an iterator, v1_iter 
    // which borrows value immutably 
    let mut v1_iter = v1.iter();

    // iter() returns an iterator of slices.
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
   // because values were borrowed immutably, it is still 
   // possible to use the vec after iteration is done
    dbg!(v1);
}
```

In the code above, the `.iter()` method that is called on the _vec_, `v1` creates an iterator, `v1_iter` which borrows value immutably. Because the values of `v1` were borrowed immutable by the iterator `v1_iter` it was still possible to use the _vec_ `v1` after the iteration by `dbg!` printing it.

**An iterator that borrows the value being iterated mutably**

```javascript
#[test]
fn iter_mut_demo() {
    let mut v1 = vec![1, 2, 3];

    // the .iter_mut() method creates an iterator, 
    // v1_iter which borrows value and can mutate it. 
    let mut v1_iter = v1.iter_mut();

    // access the first item and multiple it by 2
    let item1 = v1_iter.next().unwrap();
    *item1 = *item1 * 2;

    // access the second item and multiple it by 2
    let item2 = v1_iter.next().unwrap();
    *item2 = *item2 * 2;

    // access the third item and multiple it by 2
    let item3 = v1_iter.next().unwrap();
    *item3 = *item3 * 2;

    // end of the iteration
    assert_eq!(v1_iter.next(), None);

    // this will print out [2,4,6]
    dbg!(v1);
}
```

In the code above, the `.iter_mut()` method that is called on the _vec_, `v1` creates an iterator, `v1_iter` which borrows value mutable. That is, the value borrowed can be updated. And the code does exactly that, mutating each value iterated by multiplying it by 2. Because the values of `v1` were borrowed mutably and not owned by the iterator `v1_iter` it was still possible to use the _vec_ `v1` by printing it. Printing also confirms that the values in the _vec_ have been updated with each item multiplied by 2.

**An iterator that takes ownership of values being iterated**

```javascript
#[test]
fn into_iter_demo() {
    // the .into_iter() method creates an iterator, 
    // v1_iter which takes ownership 
    // of the values being iterated.
    let mut v1_iter = v1.into_iter();

    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);

    // If the line below is uncommented, the code won't compile anymore
    // this is because, after the iteration, v1 can no longer be used 
    // since the iteration moved ownership
    //dbg!(v1);
}
```

In the code above, the `.into_iter()` method creates an iterator, `v1_iter` which takes ownership, because of this, `v1` is no longer usable after the iteration has ownership has moved after the iteration. Uncommenting the dbg! line would lead to a compile error because of this.

### Iterator and for…in syntax

So now we have seen that Iterators in Rust can come in 3 forms depending on how ownership is handled when the iterator accesses the values of the data structure being iterated.

How does this then help us answer why the following code:

```rust
fn main() {
   let some_ints = vec![1,2,3,4,5];
   for i in some_ints {
       dbg!(i);
   }

   dbg!(some_ints);
}
```

which looks legit get flagged by the borrow checker?

The answer lies in the fact that the `for…in` syntax, by default, calls the `.into_iter()` method, which returns an iterator that takes ownership of values being iterated.

The code above is essentially the same as the following code:

```rust
fn main() {
   let some_ints = vec![1,2,3,4,5];
   for i in some_ints.into_iter() {
       dbg!(i);
   }

   dbg!(some_ints);
}
```

Compiling both will lead to the same error.

So to use the `for…in` syntax and still be able to use the underlying _vec_, after iteration, we will have to manually call either the `iter()` or `iter_mut()` method instead.

For example, using the `iter()` method with `for..in` will look like this:

```rust
fn main() {

   let some_ints = vec![1,2,3,4,5];
   for i in some_ints.iter() {
       dbg!(i);
   }

   dbg!(some_ints);
```

This time around, the code compiles fine. And if the code is executed, the contents of `some_ints` will get printed fine even after the iteration. Something that was not possible before.

Using the `iter_mut()` method on the other hand enables us to mutate the values as they are being iterated. For example:

```rust
fn main() {

   let mut some_ints = vec![1,2,3,4,5];
   for i in some_ints.iter_mut() {
       *i = *i * 2;
   }

   dbg!(some_ints);

}
```

The above code compiles fine, and when run, will print `[2,3,6,8,10]` confirming that it was possible to mutate the values while iterating.

To further explore how all the above works, check [IntoIterator and the for ... in Syntax in Rust](https://www.geekabyte.io/2022/08/intoiterator-and-for-in-syntax-in-rust.html)

### Summary

- An Iterator is a pattern that allows retrieval of elements from another data structure in sequence. In Rust, it is represented by the `Iterator` trait.
- A data structure implementing the `Iterator` trait interacts with the borrow checker based on how it handles access to the values in the data structure being iterated.

- It is possible for the iterator to borrow the value being iterated immutably. This type of iterator is created by calling `iter()`.
- It is possible for the iterator to borrow the value being iterated mutably. This type of iterator is created by calling `iter_mut()`.
- It is possible for the iterator to take ownership of the value values being iterated. This type of iterator is created by calling `into_iter()`

- By default, the `for..in` syntax calls the `into_iter()` which takes ownership of the value being iterated. This is why after iteration, the underlying variable that holds the original _vec_ becomes unusable. This is because ownership has moved away from it.
- To use the `for..in` syntax and still be able to use the variable that holds the original _vec_, after iteration, then either call `iter()` or `iter_mut()`