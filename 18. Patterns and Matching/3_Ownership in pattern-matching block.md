# 3_Ownership in pattern-matching block

Give me examples of pattern-matching blocks that enforce ownership.

___


Regarding your question about examples of pattern-matching blocks that enforce ownership, here are a few scenarios where ownership can be enforced:

1. Matching with `enum` variants:

   ```rust
   enum MyEnum {
       Variant1(String),
       Variant2(u32),
       Variant3(Vec<i32>),
   }

   let my_value = MyEnum::Variant1("Hello".to_owned());

   match my_value {
       MyEnum::Variant1(string) => {
           println!("String: {}", string);
           // Ownership of `string` is transferred into this arm
           // You can use `string` here and it is dropped at the end of the arm
       }
       _ => {}
   }

   // Error: `my_value` has moved and cannot be used anymore
   // println!("Value: {:?}", my_value);
   ```

   In this example, when matching the `Variant1` variant of the `MyEnum` enum, ownership of the inner `String` value is transferred into the match arm. You can use and manipulate the `string` variable within that arm, but outside of it, `my_value` is no longer usable.

2. Matching with ownership transfer of a variable:

   ```rust
   let value = Some("Hello".to_owned());

   match value {
       Some(owned_string) => {
           println!("String: {}", owned_string);
           // Ownership of `owned_string` is transferred into this arm
           // You can use `owned_string` here and it is dropped at the end of the arm
       }
       None => {}
   }

   // Error: `value` has moved and cannot be used anymore
   // println!("Value: {:?}", value);
   ```

   In this example, the ownership of the `value` variable is transferred into the match arm when matching the `Some` variant. The `owned_string` variable can be used within that arm, but `value` cannot be accessed afterwards.

It's important to note that ownership transfer in pattern matching depends on the specific patterns being used. In some cases, ownership may not be transferred, allowing the matched value to be accessible outside the match expression. It depends on the pattern being matched and the bindings used within the arms.