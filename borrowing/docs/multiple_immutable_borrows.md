# test_multiple_immutable_borrows

For the `test_multiple_immutable_borrows` test, we'll demonstrate how Rust allows multiple immutable borrows of a value at the same time. This is key to understanding how Rust's borrowing rules facilitate safe concurrent access to data, as long as it's only being read and not modified.

Test Outline:
1. Create a function that takes an immutable reference to an integer and returns some derived value.
2. In the test, create an integer and borrow it immutably multiple times, passing these references to the function.
3. Assert that the function returns the expected value each time.
4. Optionally, assert that the original value remains unchanged.

Let's write the Rust code for this test:

```rust
#[test]
fn test_multiple_immutable_borrows() {
    fn square(value: &i32) -> i32 {
        value.pow(2)
    }

    let x = 4;
    let first_borrow = square(&x); // First immutable borrow
    let second_borrow = square(&x); // Second immutable borrow

    assert_eq!(first_borrow, 16); // Asserting the result of the first borrow
    assert_eq!(second_borrow, 16); // Asserting the result of the second borrow
    assert_eq!(x, 4); // Asserting that x has not been modified
}
```

In this test:
- The `square` function takes an immutable reference and returns its square.
- We call `square` twice with immutable references to the same variable `x`.
- We assert that both calls to `square` return 16, which is the square of 4.
- We also assert that the value of `x` remains unchanged, ensuring that the immutable borrows did not modify the original data.

This test confirms that Rust allows multiple immutable borrows and that these borrows do not interfere with each other or alter the borrowed data.