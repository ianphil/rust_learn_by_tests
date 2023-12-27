# test_borrowing_in_loops

The `test_borrowing_in_loops` will focus on demonstrating how to use borrowing within loops in Rust. This is particularly relevant for understanding how Rust's borrowing rules apply in iterative scenarios, which is a common use case in many programs.

Test Outline:
1. Create a function that takes a reference to a collection (like a vector) and performs some operation on each element.
2. In the test, create a collection and pass a reference to it to the function in a loop.
3. Assert the expected outcome after the loop completes.

Here's an example implementation of this test:

```rust
#[test]
fn test_borrowing_in_loops() {
    fn sum(values: &[i32]) -> i32 {
        values.iter().sum()
    }

    let numbers = vec![1, 2, 3, 4, 5];
    let mut total = 0;

    for _ in 0..numbers.len() {
        total += sum(&numbers); // Borrowing the vector in each iteration
    }

    // Asserting the total sum after the loop
    // Since `sum` is called 5 times, the total should be 5 times the sum of numbers
    assert_eq!(total, 5 * (1 + 2 + 3 + 4 + 5));
}
```

In this test:
- `sum` is a function that takes an immutable reference to a slice of `i32` and returns the sum of the elements.
- We create a vector `numbers` with some integers.
- In a loop, we repeatedly call `sum` with a reference to `numbers` and accumulate the results in `total`.
- Finally, we assert that `total` is equal to 5 times the sum of the elements in `numbers`.

This test demonstrates borrowing a collection in a loop and highlights the importance of understanding the scope of borrows in iterative contexts. It shows that you can safely and repeatedly borrow data in each iteration of a loop.