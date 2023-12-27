# test_struct_borrowing
The `test_struct_borrowing` will demonstrate how borrowing works with structs in Rust, specifically how you can borrow individual fields of a struct. This is a common scenario in Rust programming and understanding it is crucial for managing complex data structures safely.

Test Outline:
1. Define a simple struct with at least one field.
2. Create a function that borrows a field of the struct (either mutably or immutably).
3. In the test, create an instance of the struct and pass a reference to its field to the function.
4. Assert that the function has the expected effect on the struct's field.

Here's an example implementation of this test:

```rust
#[test]
fn test_struct_borrowing() {
    struct MyStruct {
        value: i32,
    }

    fn increment_field(value: &mut i32) {
        *value += 1;
    }

    let mut my_struct = MyStruct { value: 10 };
    increment_field(&mut my_struct.value); // Borrowing a field of the struct

    assert_eq!(my_struct.value, 11); // Asserting that the field has been incremented
}
```

In this test:
- `MyStruct` is a simple struct with a single `i32` field named `value`.
- `increment_field` is a function that takes a mutable reference to an `i32` and increments it.
- We create an instance of `MyStruct`, then pass a mutable reference to its `value` field to `increment_field`.
- Finally, we assert that the `value` field of `MyStruct` has been incremented from 10 to 11.

This test demonstrates how Rust allows borrowing individual fields of a struct, which is a powerful feature for managing complex data structures and ensuring memory safety.