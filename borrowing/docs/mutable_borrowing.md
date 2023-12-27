# test_mutable_borrowing

Next, we'll focus on the `test_mutable_borrowing`. The purpose of this test is to demonstrate mutable borrowing in Rust. Mutable borrowing allows a function to modify a value it borrows, but under strict conditions: only one mutable borrow is allowed at a time, and no immutable borrows of the same data can coexist with a mutable borrow.

Here's the plan for the test:
1. We'll create a simple function that takes a mutable reference to an integer.
2. In the test, we'll create an integer, borrow it mutably, and pass the borrowed reference to the function.
3. The function will modify the value.
4. After the function call, we'll assert that the value has been changed as expected.

Let's write the code:

```rust
#[test]
fn test_mutable_borrowing() {
    fn increment(value: &mut i32) {
        *value += 1;
    }

    let mut x = 10;
    increment(&mut x); // Mutable borrow
    assert_eq!(x, 11); // Asserting that x has been incremented
}
```

In this test:
- `increment` is a function that takes a mutable reference to an `i32` and increments its value.
- `x` is declared as mutable with `let mut x = 10;`.
- We pass a mutable reference to `x` to the `increment` function with `&mut x`.
- The assertion `assert_eq!(x, 11)` checks that the value of `x` has been incremented by `1`, demonstrating that the mutable borrow allowed the `increment` function to modify `x`.