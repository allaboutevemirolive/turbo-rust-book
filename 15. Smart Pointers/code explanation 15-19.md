# code explanation 15-19

The code you provided demonstrates the usage of reference counting (RC) and the `Rc` type in Rust to create a simple linked list data structure. Let's go through it step by step:

1. Define the `List` enum:
   ```rust
   enum List {
       Cons(i32, Rc<List>),
       Nil,
   }
   ```
   Here, we define an enum `List` with two variants: `Cons`, representing a node in the linked list that contains an `i32` value and a reference-counted pointer (`Rc<List>`) to the next node, and `Nil`, representing the end of the list.

2. Import necessary items:
   ```rust
   use crate::List::{Cons, Nil};
   use std::rc::Rc;
   ```
   This code imports the `Cons` and `Nil` variants from the `List` enum, as well as the `Rc` type from the `std::rc` module. `Rc` stands for "reference count" and allows multiple ownership of data by keeping track of the number of references to a value.

3. Define the `main` function:
   ```rust
   fn main() {
       // ...
   }
   ```
   The `main` function is the entry point of the program.

4. Create a linked list:
   ```rust
   let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
   ```
   In this line, we create a linked list with three nodes: 5 -> 10 -> Nil. The `Rc::new` function wraps the value `Cons(5, Rc::new(Cons(10, Rc::new(Nil))))` in an `Rc` smart pointer. The `Rc` keeps track of the number of references to the list. At this point, `a` is the owner of the list.

5. Print the reference count of `a`:
   ```rust
   println!("count after creating a = {}", Rc::strong_count(&a));
   ```
   This line prints the current reference count of `a` using `Rc::strong_count`. Since `a` is the only reference to the list, the count will be 1.

6. Create a new node and share ownership:
   ```rust
   let b = Cons(3, Rc::clone(&a));
   ```
   Here, we create a new node `Cons(3, Rc::clone(&a))`. This node has a value of 3 and shares ownership of the existing list `a` by cloning the `Rc` pointer using `Rc::clone`. The reference count of `a` will increase to 2.

7. Print the reference count of `a` again:
   ```rust
   println!("count after creating b = {}", Rc::strong_count(&a));
   ```
   Since `b` now also owns the list, the reference count of `a` will be 2.

8. Create a new scope and another shared node:
   ```rust
   {
       let c = Cons(4, Rc::clone(&a));
       println!("count after creating c = {}", Rc::strong_count(&a));
   }
   ```
   Here, we introduce a new scope and create a node `Cons(4, Rc::clone(&a))`. Similar to `b`, `c` shares ownership of the list by cloning the `Rc` pointer. Within this scope, the reference count of `a` will be 3. After the scope ends, `c` goes out of scope, and its ownership is released. The reference count of `a` returns to 

2.

9. Print the reference count of `a` one last time:
   ```rust
   println!("count after c goes out of scope = {}", Rc::strong_count(&a));
   ```
   Since `c` has gone out of scope, the reference count of `a` will be 2 again.

Overall, this code demonstrates how reference counting with `Rc` allows multiple ownership of data and keeps track of the number of references to the data. It also showcases the ability to share ownership of a data structure and automatically deallocate the memory when the last reference goes out of scope.