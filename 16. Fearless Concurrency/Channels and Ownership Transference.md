# Channels and Ownership Transference

The ownership rules play a vital role in message sending because they help you write safe, concurrent code.

Preventing errors in concurrent programming is the advantage of thinking about ownership throughout your Rust programs.

Let’s do an experiment to show how channels and ownership work together to prevent problems: we’ll try to use a `val` value in the spawned thread after we’ve sent it down the channel.

Try compiling the code in Listing 16-9 to see why this code isn’t allowed:

Filename: src/main.rs

This code does not compile!

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

Listing 16-9: Attempting to use `val` after we’ve sent it down the channel

Here, we try to print `val` after we’ve sent it down the channel via `tx.send`.

Allowing this would be a bad idea: once the value has been sent to another thread, that thread could modify or drop it before we try to use the value again.

Potentially, the other thread’s modifications could cause errors or unexpected results due to inconsistent or nonexistent data.

However, Rust gives us an error if we try to compile the code in Listing 16-9:

```rust
$ cargo run
   Compiling message-passing v0.1.0 (file:///projects/message-passing)
error[E0382]: borrow of moved value: `val`
  --> src/main.rs:10:31
   |
8  |         let val = String::from("hi");
   |             --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
9  |         tx.send(val).unwrap();
   |                 --- value moved here
10 |         println!("val is {}", val);
   |                               ^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
error: could not compile `message-passing` due to previous error
```

Our concurrency mistake has caused a compile time error.

The `send` function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it.

This stops us from accidentally using the value again after sending it; the ownership system checks that everything is okay.


sub-topic:

[[Sending Multiple Values and Seeing the Receiver Waiting]]