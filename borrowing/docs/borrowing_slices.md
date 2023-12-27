# test_borrowing_slices

The `test_borrowing_slices` test is intended to demonstrate how to borrow slices of collections like arrays or vectors in Rust. Slices are a view into a contiguous sequence of elements in a collection and are a common way to access a portion of an array or vector without taking ownership of the whole collection.

Test Outline:
1. Create a function that takes a slice of an array or vector and performs an operation on it.
2. In the test, create an array or vector and then create a slice of it to pass to the function.
3. Assert the expected outcome based on the operation performed by the function.

Here's an example implementation:

```rust
#[test]
fn test_borrowing_slices() {
    fn sum_of_slice(slice: &[i32]) -> i32 {
        slice.iter().sum()
    }

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice = &numbers[2..5]; // Borrowing a slice of the array

    let sum = sum_of_slice(slice);
    assert_eq!(sum, 3 + 4 + 5); // Asserting the sum of the slice
}
```

In this test:
- `sum_of_slice` is a function that takes a slice of `i32` and returns the sum of its elements.
- We create an array `numbers` and then create a slice of it using `&numbers[2..5]`. This slice includes elements at indices 2, 3, and 4 (slices are half-open ranges in Rust, meaning they include the start index but exclude the end index).
- We call `sum_of_slice` with this slice and assert that the result is equal to the sum of the elements in the slice (`3 + 4 + 5`).

This test demonstrates borrowing a slice of an array, which is a very efficient way to pass part of an array to a function without copying the data. It also illustrates Rust's safety in handling slices, ensuring that the slice does not outlive the array it references.