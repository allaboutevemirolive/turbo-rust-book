[Link](https://www.becomebetterprogrammer.com/rust-iter-vs-iter_mut-vs-into_iter/)

July 10, 2022 by [Andres Reales](https://www.becomebetterprogrammer.com/author/arealesramirez8606/ "View all posts by Andres Reales")

![](https://www.becomebetterprogrammer.com/wp-content/uploads/2022/07/Rust-Difference-Between-iter-iter_mut-into_iter.png?ezimgfmt=rs%3Adevice%2Frscb2-1)

Rust iterators are data structures that contain a sequence of objects and allow the programmer to _iterate_ them efficiently. However, like most things in Rust, iterators have a steep learning curve. The primary functions and concepts can be daunting, but don’t worry, we are here to do our absolute best to explain them.

**All three methods `iter()`, `into_iter()` and `iter_mut()` ultimately iterate the iterator. They differ by the handle of each iterated object _T_:**

- **`iter()` yields `&T`** – **immutable references.**
- **`iter_mut()` yields, as the name suggests, `&mut T` – mutable references.**
- **`into_iter()` yields any of T (moved value), `&T` or `&mut T` depending on the usage.**

If the differences between iter(), into_iter(), and iter_mut() are not still clear, keep reading this post. You will learn more about each of these methods, helping you understand them by looking at easy-to-follow examples.

Table of Contents

- [Understanding iter() and Basic Iterator Functionality](https://www.becomebetterprogrammer.com/rust-iter-vs-iter_mut-vs-into_iter/#Understanding_iter_and_Basic_Iterator_Functionality "Understanding iter()  and Basic Iterator Functionality")
    - [Lazy Evaluation and Consuming Iterators](https://www.becomebetterprogrammer.com/rust-iter-vs-iter_mut-vs-into_iter/#Lazy_Evaluation_and_Consuming_Iterators "Lazy Evaluation and Consuming Iterators")
- [Understanding iter_mut()](https://www.becomebetterprogrammer.com/rust-iter-vs-iter_mut-vs-into_iter/#Understanding_iter_mut "Understanding iter_mut()")
- [Understanding into_iter()](https://www.becomebetterprogrammer.com/rust-iter-vs-iter_mut-vs-into_iter/#Understanding_into_iter "Understanding into_iter()")
- [Conclusion](https://www.becomebetterprogrammer.com/rust-iter-vs-iter_mut-vs-into_iter/#Conclusion "Conclusion")

## Understanding **`iter()`** and Basic Iterator Functionality

[iter()](https://doc.rust-lang.org/stable/std/iter/) is the most common method out of the three, and most examples online use it. It allows the programmer to iterate each value as an immutable reference. Let us look at a small example:

```rust
let fruits = vec!["Banana", "Apple", "Grapes", "Pineapple"];
    println!(
        "{}",
        fruits
            .iter()
            .map(|fruit_name: &&str| fruit_name.len())
            .fold(0, |accumulated, fruit_name_length| {
                accumulated + fruit_name_length
            })
    ); // Should print 26
```

This example demonstrates a couple of things; let us go over it one line at a time.

The first line defines a new variable named **`fruits`** and assigns to it a vector (**`Vec<&str>`**) of four fruit names using the **`vec!`** Macro.

The second and third lines order Rust to print the output of the statement that starts in the fourth line.

Here (line 5) we see the call to the **`iter()`** method, which allows us to call the first actual iterator method – **`map`**. [**`map`**](https://doc.rust-lang.org/stable/std/iter/struct.Map.html) is one of many methods that is called on an iterator and returns a new iterator.

The name “_map”_ is not specific to Rust and is used in many other programming languages for a function that _maps_ each element in a sequence of elements to another element (most commonly creates a modified version of the original element) in a new sequence.

In this simple example, I use the **`map`** method to _map_ each **`fruit_name`** element in the **`fruits`** vector to its length. Notice the parameter that **`map`** receives – that is called a _closure_. A closure is an anonymous function (a function with no name) that can be stored in a variable or passed as an argument to another function.

I passed a closure to **`map`** that receives as a parameter (between the **_|_** characters) a **`fruit_name`** of type **`&&str`**. The type **`&&str`** is expected; we are iterating **`&str`** elements using **`iter()`** which yields immutable references to the elements in the iterator. The closure returns the length of **`fruit_name`**.

The next call is to the [fold](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.fold) method, which iterates the entire iterator and calculates a cumulating single value. The **`fold`** method receives two parameters:

1. The initial accumulated value (the starting value; the value built upon).
2. A closure that executes for each element in the iterator and receives two parameters:
    1. The accumulated value so far.
    2. The current element.

In this example, the sum of all fruit lengths is calculated, with an initial value of 0. In this case, it would also make sense to use the [reduce](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.reduce) method that uses the first element of the iterator instead of taking the initial accumulated value as a parameter.

### Lazy Evaluation and Consuming Iterators

The call to the **`fold`** method is special since it _consumes_ the entire iterator. To understand what _consuming_ means, we first must understand that Rust iterators are lazily evaluated, meaning no actual calculation is performed until the result is requested. A classic example of this mechanism is the [next](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#tymethod.next) method. Let’s look at how it is used:

```rust
println!(
        "{}",
        fruits
            .iter()
            .next()
            .unwrap()
    ); // Should print “Banana”
```

The **`next`** method consumes the _first_ element of the iterator, meaning it triggers its calculation and removes it from the iterator. It is important to note that **`next`** returns an **`Option`** (more about the **`Option`** enum [here](https://doc.rust-lang.org/std/option/)), which might contain **`None`** if **`next`** is called on an empty iterator.

## Understanding iter_mut()

[iter_mut()](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.iter_mut) is the least used method out of the three. It allows the programmer to iterate each element as a mutable reference. Let us look at a small example:

```rust
let mut numbers = vec![1, 2, 3, 4];
numbers.iter_mut().for_each(|n| *n += 1);
println!("{:?}", numbers); // Should print [2, 3, 4, 5]
```

In this example, we define a mutable vector of numbers and iterate it using **`iter_mut()`**. The second line also introduces a new method: **`for_each()`**. **`for_each()`** is the one-line version of iterating using the **`for`** keyword like so:

```rust
for n in numbers.iter_mut() {
        *n += 1;
}
```

**`for_each()`** receives a parameter which is a closure to execute the _for each_ element in the iterator. Since we are using **`iter_mut`**, we are iterating mutable references, which means the closure can modify every number **`n`** in the original vector using dereferencing_._

It is important to note that since iterators are lazy, using methods like **`map`**, **`fold`_,_** and **`reduce`** without consuming the iterators won’t change the elements of the vector.

## Understanding into_iter()

**`into_iter()`** is a method whose name comes from the trait **`IntoIterator`** which we must understand before we continue. **`IntoIterator`** is a trait that should be implemented when the developer wants to specify how a certain type should be converted _into an iterator_. It is useful when the type represents some sort of collection of data and most notably allows for an iteration over objects of the type in a **_for_** loop syntax.

Let’s look at an example before we continue to **`into_iter()`**.

The type **`Vec<T>`** implements **`IntoIterator`** for three cases:

- **`impl<T> IntoIterator for Vec<T>`**
- **`impl<'a, T> IntoIterator for &'a Vec<T>`**
- **`impl<'a, T> IntoIterator for &'a mut Vec<T>`**

The first implementation allows for iterating by value (yields **`T`**s), it implements **`IntoIterator`** for a **`Vec`** that holds elements of some generic type **`T`**.

The second implementation allows for iterating by reference (yields **`&T`**s), it implements **`IntoIterator`** for a reference of generic lifetime **`‘a`** to a **`Vec`** that holds elements of some generic type **`T`**.

The third implementation is similar to the second implementation, but with a generic mutable reference instead.

Those three implementations allow the following code snippet to compile:

```rust
let by_value = vec![1, 2, 3];
    for i in by_value {
        println!("{}", i);
    }
    // println!("{:?}", by_value);

    let by_reference = vec![1, 2, 3];

    for i in &by_reference {
        println!("{}", i);
    }

    println!("{:?}", by_reference);

    let mut by_mutable_reference = vec![1, 2, 3];
    for i in &mut by_mutable_reference {
        *i += 1;
    }
    println!("{:?}", by_mutable_reference);
```

This snippet is long so I’ll explain it in multiple parts.

In the first part (lines 1 to 5), we iterate the **`by_value`** vector _by value_ and print its elements. It is important to note that since we are iterating the elements _by value,_ the elements are moved and so it is impossible to print its elements after the **`for`** loop.

In the second part (lines 7 to 13), we iterate the **`by_reference`** vector _by reference_ and print its elements. Now, since we are iterating the elements _by reference,_ the elements are _not_ moved and so printing the elements again after the **`for`** loop is indeed possible.

In the third and last part (lines 15 to 19), we iterate the **`by_mutable_reference`** vector _by mutable reference_, add 1 to every one of its elements and then we can print the vector as expected.

In all three cases, **`into_iter`** is called _implicitly_ and in all three cases, the elements are handled differently – **`i32`**, **`&i32`_,_** and **`&mut i32`** (in order).

## Conclusion

In conclusion, **`iter()`**, **`iter_mut()`** and **`into_iter()`** are ultimately methods that allow the programmer to _iterate_ over a collection of data.

- **`iter()`** provides iteration over immutable references (**`&T`**).
- **`iter_mut()`** provides iteration over mutable references (**`&mut T`**).
- **`into_iter()`** allows for iteration over any of the moved values (**`T`**), immutable references, or mutable references. In addition, it is also implicitly called (when possible) when using a **`for`** loop to iterate the collection.
