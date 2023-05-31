# turbo-rust-book
A blank space between each sentence to make reading `The Rust Programming Language` simpler.

### First release version: 
- 1.0.0

### Current release version: 
- 1.0.5

___

Code explanation on version: 
- 1.0.0

```rust
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

fn main() {
    let input_file    = "input.txt";
    let output_file   = "7.3. Paths for Referring to an Item in the Module Tree.md";
    let output_file = output_file.replace(" ", "_").to_lowercase();

    let mut new_text = String::new();

    let file = File::open(input_file).expect("Unable to open input file");
    let mut buf_reader = BufReader::new(file);
    let mut data = String::new();
    buf_reader.read_to_string(&mut data).expect("Unable to read input file");

    data = data.replace(". ", ".\n");

    let mut data_chars = data.chars().peekable();
    while let Some(c) = data_chars.next() {
        match c {
            '{' => new_text.push('{'),
            '}' => new_text.push('}'),
            '.' => {
                new_text.push('.');
                if let Some(&next_char) = data_chars.peek() {
                    if next_char != 'r' && !next_char.is_digit(10) {
                        new_text.push('\n');
                    }
                }
            }
            _ => new_text.push(c),
        }
    }

    let output_file = File::create(output_file).expect("Unable to create output file");
    let mut buf_writer = BufWriter::new(output_file);
    buf_writer
        .write_all(new_text.as_bytes())
        .expect("Unable to write to output file");
}

```

This code processes a text file containing multiple sentences and formats it by adding a newline after each sentence, making it easier to read. 

Let's go through the code line by line:


```rust
use std::fs::File; 
```
Imports the File struct from the `std::fs` module, which provides access to an open file on the filesystem.

References:
- https://doc.rust-lang.org/std/fs/struct.File.html



```rust
use std::io::{BufReader, BufWriter, Read, Write};: 
```
Imports `BufReader`, `BufWriter`, `Read`, and `Write` traits from the `std::io` module. These are used for buffered reading and writing of files.

References:
- https://doc.rust-lang.org/std/io/struct.BufWriter.html
- https://doc.rust-lang.org/std/io/struct.BufReader.html



```rust
fn main() {
```
Defines the main function, which is the entry point of the program.

References:
- https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
- https://subscription.packtpub.com/book/programming/9781788390637/1/ch01lvl1sec12/main-function


 
```rust
let input_file = "input.txt";
```
Declares a variable `input_file` and assigns it the name of the input file.


 
```rust
let output_file = "output.md";
```
Declares a variable `output_file` and assigns it the name of the output file.



```rust
let output_file = output_file.replace(" ", "_").to_lowercase();
```
Replaces spaces with underscores and converts the output file name to lowercase.



```rust
let mut new_text = String::new();
```
Creates a mutable String variable `new_text` to store the formatted text.



```rust
let file = File::open(input_file).expect("Unable to open input file");
```
Opens the input file for reading and panics with an error message if it fails.



```rust
let mut buf_reader = BufReader::new(file);
```
Creates a buffered reader for the input file.

References:
- https://doc.rust-lang.org/std/io/struct.BufReader.html



```rust
let mut data = String::new();
```
Creates a mutable String variable data to store the contents of the input file.



```rust
buf_reader.read_to_string(&mut data).expect("Unable to read input file");
```
Reads the entire input file into the data variable and panics with an error message if it fails.

References:
- https://rust-lang-nursery.github.io/rust-cookbook/file/read-write.html

 

```rust
data = data.replace(". ", ".\n");
```
Replaces occurrences of `". "` with `".\n"` in the data variable, adding a newline after each sentence.


 
```rust
let mut data_chars = data.chars().peekable();
```
Creates a mutable iterator over the characters in data that allows peeking at the next character without consuming it.



```rust
while let Some(c) = data_chars.next() { ... }
```
Iterates over each character in `data_chars`.


 
```rust
match c { ... }
```
A match statement that checks the current character `c` against different cases.

References:
- https://doc.rust-lang.org/rust-by-example/flow_control/match.html
- https://doc.rust-lang.org/book/ch06-02-match.html



```rust
'{' => new_text.push('{'),
```
If `c` is `'{'`, it appends it to `new_text`.



```rust
'}' => new_text.push('}'),
```
If `c` is `'}'`, it appends it to `new_text`.



```rust
'.' => { ... }
```
If `c` is `'.'`, it appends it to `new_text` and checks the next character to determine if a newline should be added.



```rust
new_text.push('.');
```
Appends the period `('.')` to the `new_text` variable.



```rust
if let Some(&next_char) = data_chars.peek() {
```  
Peeks at the next character in the `data_chars` iterator without consuming it. If there is a next character, it binds the value to `next_char`.



```rust
if next_char != 'r' && !next_char.is_digit(10) {
```
Checks if the next character is not `'r'` and not a digit (base 10). This condition is used to avoid adding a newline after a period that is part of an abbreviation (e.g., "Mr.") or a number (e.g., "3.14").



```rust
new_text.push('\n');
```
If the condition in step 3 is true, appends a newline character `('\n')` to `new_text`, effectively separating sentences with a blank space.



```rust
_ => new_text.push(c),
```
For any other character, it appends it to `new_text`.



```rust
let output_file = File::create(output_file).expect("Unable to create output file");
```
Creates the output file for writing and panics with an error message if it fails.

References:
- https://doc.rust-lang.org/std/fs/struct.File.html
- https://rust-lang-nursery.github.io/rust-cookbook/file/read-write.html



```rust
let mut buf_writer = BufWriter::new(output_file);
```
Creates a buffered writer for the output file.

References:
- https://doc.rust-lang.org/std/io/struct.BufWriter.html



```rust
buf_writer.write_all(new_text.as_bytes()).expect("Unable to write to output file");
```
Writes the formatted text `(new_text)` to the output file and panics with an error message if it fails.

References:
- https://rust-lang-nursery.github.io/rust-cookbook/file/read-write.html


