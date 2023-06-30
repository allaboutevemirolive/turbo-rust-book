# Objects Contain Data and Behavior

The book *Design Patterns: Elements of Reusable Object-Oriented Software* by Erich Gamma, Richard Helm, Ralph Johnson, and John Vlissides (Addison-Wesley Professional, 1994), colloquially referred to as *The Gang of Four book*, is a catalog of object-oriented design patterns.


___

It defines OOP this way:

Object-oriented programs are made up of objects.

An object packages both data and the procedures that operate on that data.

The procedures are typically called methods or operations.
___


Using this definition, Rust is object-oriented: structs and enums have data, and `impl` blocks provide methods on structs and enums.

Even though structs and enums with methods aren’t *called* objects, they provide the same functionality, according to the Gang of Four’s definition of objects.



[[Encapsulation that Hides Implementation Details]]

