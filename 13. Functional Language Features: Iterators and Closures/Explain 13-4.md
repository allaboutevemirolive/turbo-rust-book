Explain to me how this code works in details

```rust
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}
```

Output :

```rust
Before defining closure: [1, 2, 3]
Before calling closure: [1, 2, 3]
From closure: [1, 2, 3]
After calling closure: [1, 2, 3]
```

___

Certainly! Let's walk through the code step by step:

1. The code starts with the `main` function, which is the entry point of the program.

2. Inside the `main` function, a vector named `list` is defined and initialized with the values `[1, 2, 3]`. This vector contains a series of integers.

3. The `println!` macro is used to print the content of the vector `list` before defining the closure. It will display the message "Before defining closure:" followed by the contents of `list`.

4. The next line defines a closure named `only_borrows` using the `||` syntax. This closure takes no arguments and has an empty parameter list.

5. Inside the closure body, the `println!` macro is used to print the content of the vector `list`. It will display the message "From closure:" followed by the contents of `list`. Note that the closure captures the `list` variable from its surrounding scope.

6. The `println!` macro is used again to print the content of the vector `list`, this time before calling the closure. It will display the message "Before calling closure:" followed by the contents of `list`.

7. The `only_borrows()` syntax calls the closure `only_borrows`. Since the closure captures the `list` variable, it can access and print the contents of `list`.

8. After calling the closure, another `println!` macro is used to print the content of the vector `list`. It will display the message "After calling closure:" followed by the contents of `list`.

In summary, this code demonstrates the usage of a closure that borrows a variable (`list`) from its surrounding scope. The closure is defined using the `||` syntax and captures the `list` variable, allowing it to access and print the contents of `list`. The closure is called within the `main` function, and before and after calling the closure, the content of `list` is printed to show how the closure can access the borrowed variable.