# Bounded Parametric Polymorphism for Traits

Bounded parametric polymorphism with traits:

Traits themselves can also utilize bounded parametric polymorphism by using associated types and trait bounds. By defining associated types with trait bounds, you can enforce constraints on the concrete types that implement the trait.

Here's an example of a trait Collection with an associated type Item that must implement the Clone trait:

```rust
trait Collection {
    type Item: Clone;

    fn add(&mut self, item: Self::Item);
}

struct MyCollection<T: Clone> {
    items: Vec<T>,
}

impl<T: Clone> Collection for MyCollection<T> {
    type Item = T;

    fn add(&mut self, item: Self::Item) {
        self.items.push(item);
    }
}

fn main() {
    let mut collection = MyCollection { items: Vec::new() };
    collection.add("Hello".to_string());
    collection.add(42.to_string());
}
```