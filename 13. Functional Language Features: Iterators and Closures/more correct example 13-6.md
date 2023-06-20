```rust
fn main() {
    let vector = vec![1, 2, 3];
    
    let closure = move || {
        let mut cloned_vector = vector.clone();
        cloned_vector.push(4);
        cloned_vector[0] = 10;
        println!("{:?}", cloned_vector);
    };
    
    closure();  // Call the closure
}
```
