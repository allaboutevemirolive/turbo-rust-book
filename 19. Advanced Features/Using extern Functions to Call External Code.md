# Using extern Functions to Call External Code

Sometimes, your Rust code might need to interact with code written in another language.

For this, Rust has the keyword extern that facilitates the creation and use of a Foreign Function Interface (FFI).

An FFI is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions.



Listing 19-8 demonstrates how to set up an integration with the abs function from the C standard library.

Functions declared within extern blocks are always unsafe to call from Rust code.

The reason is that other languages don’t enforce Rust’s rules and guarantees, and Rust can’t check them, so responsibility falls on the programmer to ensure safety.



Filename: src/main.rs

extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
Listing 19-8: Declaring and calling an extern function defined in another language

Within the extern "C" block, we list the names and signatures of external functions from another language we want to call.

The "C" part defines which application binary interface (ABI) the external function uses: the ABI defines how to call the function at the assembly level.

The "C" ABI is the most common and follows the C programming language’s ABI.



Calling Rust Functions from Other Languages
We can also use extern to create an interface that allows other languages to call Rust functions.

Instead of creating a whole extern block, we add the extern keyword and specify the ABI to use just before the fn keyword for the relevant function.

We also need to add a #[no_mangle] annotation to tell the Rust compiler not to mangle the name of this function.

Mangling is when a compiler changes the name we’ve given a function to a different name that contains more information for other parts of the compilation process to consume but is less human readable.

Every programming language compiler mangles names slightly differently, so for a Rust function to be nameable by other languages, we must disable the Rust compiler’s name mangling.



In the following example, we make the call_from_c function accessible from C code, after it’s compiled to a shared library and linked from C:

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
This usage of extern does not require unsafe.



Accessing or Modifying a Mutable Static Variable
In this book, we’ve not yet talked about global variables, which Rust does support but can be problematic with Rust’s ownership rules.

If two threads are accessing the same mutable global variable, it can cause a data race.



In Rust, global variables are called static variables.

Listing 19-9 shows an example declaration and use of a static variable with a string slice as a value.



Filename: src/main.rs

static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
Listing 19-9: Defining and using an immutable static variable

Static variables are similar to constants, which we discussed in the “Differences Between Variables and Constants” section in Chapter 3.

The names of static variables are in SCREAMING_SNAKE_CASE by convention.

Static variables can only store references with the 'static lifetime, which means the Rust compiler can figure out the lifetime and we aren’t required to annotate it explicitly.

Accessing an immutable static variable is safe.



A subtle difference between constants and immutable static variables is that values in a static variable have a fixed address in memory.

Using the value will always access the same data.

Constants, on the other hand, are allowed to duplicate their data whenever they’re used.

Another difference is that static variables can be mutable.

Accessing and modifying mutable static variables is unsafe.

Listing 19-10 shows how to declare, access, and modify a mutable static variable named COUNTER.



Filename: src/main.rs

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
Listing 19-10: Reading from or writing to a mutable static variable is unsafe

As with regular variables, we specify mutability using the mut keyword.

Any code that reads or writes from COUNTER must be within an unsafe block.

This code compiles and prints COUNTER: 3 as we would expect because it’s single threaded.

Having multiple threads access COUNTER would likely result in data races.



With mutable data that is globally accessible, it’s difficult to ensure there are no data races, which is why Rust considers mutable static variables to be unsafe.

Where possible, it’s preferable to use the concurrency techniques and thread-safe smart pointers we discussed in Chapter 16 so the compiler checks that data accessed from different threads is done safely.



Implementing an Unsafe Trait
We can use unsafe to implement an unsafe trait.

A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify.

We declare that a trait is unsafe by adding the unsafe keyword before trait and marking the implementation of the trait as unsafe too, as shown in Listing 19-11.



unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
Listing 19-11: Defining and implementing an unsafe trait

By using unsafe impl, we’re promising that we’ll uphold the invariants that the compiler can’t verify.



As an example, recall the Sync and Send marker traits we discussed in the “Extensible Concurrency with the Sync and Send Traits” section in Chapter 16: the compiler implements these traits automatically if our types are composed entirely of Send and Sync types.

If we implement a type that contains a type that is not Send or Sync, such as raw pointers, and we want to mark that type as Send or Sync, we must use unsafe.

Rust can’t verify that our type upholds the guarantees that it can be safely sent across threads or accessed from multiple threads; therefore, we need to do those checks manually and indicate as such with unsafe.



Accessing Fields of a Union
The final action that works only with unsafe is accessing fields of a union.

A union is similar to a struct, but only one declared field is used in a particular instance at one time.

Unions are primarily used to interface with unions in C code.

Accessing union fields is unsafe because Rust can’t guarantee the type of the data currently being stored in the union instance.

You can learn more about unions in the Rust Reference.



When to Use Unsafe Code
Using unsafe to take one of the five actions (superpowers) just discussed isn’t wrong or even frowned upon.

But it is trickier to get unsafe code correct because the compiler can’t help uphold memory safety.

When you have a reason to use unsafe code, you can do so, and having the explicit unsafe annotation makes it easier to track down the source of problems when they occur.

