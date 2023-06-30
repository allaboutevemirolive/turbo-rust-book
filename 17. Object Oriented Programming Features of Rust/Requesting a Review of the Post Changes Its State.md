# Requesting a Review of the Post Changes Its State

Next, we need to add functionality to request a review of a post, which should change its state from `Draft` to `PendingReview`.

Listing 17-15 shows this code:

Filename: src/lib.rs

```rust
impl Post {
    // --snip--
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

Listing 17-15: Implementing `request_review` methods on `Post` and the `State` trait

We give `Post` a public method named `request_review` that will take a mutable reference to self.

Then we call an internal `request_review` method on the current state of `Post`, and this second `request_review` method consumes the current state and returns a new state.


We add the `request_review` method to the `State` trait; all types that implement the trait will now need to implement the `request_review` method.

Note that rather than having `self`, `&self`, or `&mut self` as the first parameter of the method, we have `self: Box<Self>`.

This syntax means the method is only valid when called on a `Box` holding the type.

This syntax takes ownership of `Box<Self>`, invalidating the old state so the state value of the `Post` can transform into a new state.



To consume the old state, the `request_review` method needs to take ownership of the state value.

This is where the `Option` in the state field of `Post` comes in: we call the `take` method to take the `Some` value out of the `state` field and leave a `None` in its place, because Rust doesn’t let us have unpopulated fields in structs.

This lets us move the `state` value out of `Post` rather than borrowing it.

Then we’ll set the post’s `state` value to the result of this operation.



We need to set `state` to `None` temporarily rather than setting it directly with code like `self.state = self.state.request_review();` to get ownership of the state value.

This ensures `Post` can’t use the old `state` value after we’ve transformed it into a new state.



The `request_review` method on `Draft` returns a new, boxed instance of a new `PendingReview` struct, which represents the state when a post is waiting for a review.

The `PendingReview` struct also implements the `request_review` method but doesn’t do any transformations.

Rather, it returns itself, because when we request a review on a post already in the `PendingReview` state, it should stay in the `PendingReview` state.



Now we can start seeing the advantages of the state pattern: the `request_review` method on `Post` is the same no matter its `state` value.

Each state is responsible for its own rules.



We’ll leave the `content` method on `Post` as is, returning an empty string slice.

We can now have a `Post` in the `PendingReview` state as well as in the `Draft` state, but we want the same behavior in the `PendingReview` state.

Listing 17-11 now works up to line 10!


[[Adding approve to Change the Behavior of content]]
