# Stack and Heap in rust

Stack and Heap in rust

What is heap and stack in rust

___

In Rust, the heap and the stack are two different areas of memory used for storing data.

1. Stack:
   - The stack is a region of memory used for storing local variables and function call information.
   - Memory allocation and deallocation on the stack are handled automatically by the compiler.
   - Stack memory follows a Last-In-First-Out (LIFO) order, where the most recently allocated memory is the first to be deallocated.
   - Stack memory is generally faster to allocate and deallocate compared to heap memory.
   - Stack memory has a fixed size determined at compile time, and it is typically limited in size.

2. Heap:
   - The heap is a region of memory used for dynamic memory allocation.
   - Memory allocation and deallocation on the heap are explicitly controlled by the programmer.
   - Heap memory is used for storing data structures with a flexible or unknown size, such as dynamic arrays or objects created at runtime.
   - Heap memory does not have a specific order of allocation or deallocation, and memory can be allocated and deallocated at any time during the program's execution.
   - Heap memory is larger in size compared to the stack and can grow as needed (up to the system's available memory).
   - Managing heap memory manually can be more complex and error-prone compared to stack memory.

In Rust, memory management is primarily handled through ownership, borrowing, and lifetimes. The ownership system ensures that stack-allocated variables are automatically dropped when they go out of scope, releasing their associated resources. Memory allocated on the heap is typically managed using smart pointers such as `Box<T>`, `Rc<T>`, or `Arc<T>`, which provide ownership and reference counting mechanisms.

By leveraging the stack for small and short-lived data and the heap for larger or dynamically sized data, Rust ensures memory safety without sacrificing performance.