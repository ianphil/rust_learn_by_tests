# Borrowing

To effectively demonstrate the concepts of pointers and borrowing in Rust, we should consider a variety of scenarios to cover the fundamental aspects and common use cases. Here's a proposed set of tests, along with their objectives:

1. [**test_immutable_borrowing**](docs/immutable_borrowing.md)
   - Demonstrates basic immutable borrowing, showing how a function can borrow a value immutably for read-only access.

2. [**test_mutable_borrowing**](docs/mutable_borrowing.md)
   - Explores mutable borrowing, illustrating how a function can borrow a value mutably to modify it.

3. **test_multiple_immutable_borrows**
   - Highlights the rule that multiple immutable borrows are allowed at the same time.

4. **test_mutable_and_immutable_borrowing_conflict**
   - Shows that Rust disallows simultaneous mutable and immutable borrowing, demonstrating the borrow checker's role in preventing data races.

5. **test_lifetime_of_borrowed_reference**
   - Focuses on lifetimes, ensuring a borrowed reference does not outlive the data it references.

6. **test_struct_borrowing**
   - Applies borrowing concepts to struct fields, demonstrating borrowing individual fields of a struct.

7. **test_borrowing_in_loops**
   - Shows how to use borrowing within loops, a common scenario where borrowing rules need careful consideration.

8. **test_dangling_pointer_prevention**
   - Illustrates Rust’s prevention of dangling pointers through its borrowing and ownership rules.

9. **test_borrowing_slices**
   - Demonstrates borrowing parts of a collection, like slices of an array or vector.

10. **test_function_returning_borrowed_reference**
    - Explores the concept of returning borrowed references from functions and the associated lifetime considerations.

This set of 10 tests should provide a comprehensive overview of pointers and borrowing in Rust, covering basic to more complex scenarios. Each test will be designed to not only demonstrate a specific aspect of the borrowing system but also to showcase common pitfalls and best practices. Let's proceed with writing the code for these tests.