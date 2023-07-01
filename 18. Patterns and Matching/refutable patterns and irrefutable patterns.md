# refutable patterns and irrefutable patterns

Give me exhaustive list of refutable patterns and irrefutable patterns in rust

___

Certainly! Here's an exhaustive list of both refutable and irrefutable patterns in Rust:

Refutable Patterns:

1. Constants and literals: `42`, `"hello"`, etc.
2. Variables: `x`, `name`, etc.
3. Destructuring patterns:
   - Tuples: `(x, y)`, `(1, _)`, etc.
   - Enums: `Some(x)`, `Result::Ok(x)`, etc.
   - Structs: `Point { x, y }`, `Person { name, age, .. }`, etc.
4. Slice patterns: `[x, y, z]`, `[head, tail @ ..]`, etc.
5. Reference patterns: `&x`, `&mut x`, etc.
6. Range patterns: `1..5`, `a..=z`, etc.
7. Pattern guards: `x if x > 0`, `Some(x) if x > 0`, etc.
8. Or patterns: `Some(x) | None`, `Ok(x) | Err(e)`, etc.
9. Macro patterns: `vec![x, y, ..]`, `println!("{:?}", x)`, etc.

Irrefutable Patterns:

1. Variables: `x`, `name`, etc.
2. Wildcard: `_`
3. Ignored references: `_`, `&x`, `&_`, etc.
4. Destructuring patterns:
   - Tuples: `(x, _)`, `(_, y)`, etc.
   - Enums: `Some(_)`, `Result::Ok(_)`, etc.
   - Structs: `Point { .. }`, `Person { name, .. }`, etc.
5. Reference patterns: `&x`, `&mut x`, etc.
6. Slice patterns: `[x, y, ..]`, `[..]`, etc.
7. Range patterns: `..`, `..=10`, etc.
8. Macro patterns: `vec![..]`, `println!("{:?}", _)`, etc.

Remember that *refutable* patterns are used in `match` expressions, `if let` expressions, and function arguments, where you need to handle specific cases explicitly. 

*Irrefutable* patterns are typically used in `let` statements and function arguments, where you want to bind variables to values without explicitly handling different cases.