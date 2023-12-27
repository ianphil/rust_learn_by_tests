# test_immutable_borrowing

1. **Objective**: This test demonstrates the concept of immutable borrowing in Rust. It shows how you can pass a reference to a value to a function, allowing the function to read the value without taking ownership.

2. **Function `double_value`**:
   - This is a helper function within the test.
   - It takes an immutable reference to an `i32` (`&i32`).
   - It returns an `i32` which is double the value of the referenced `i32`.

3. **Setting Up the Test**:
   - Define an integer variable `x` with a value of `10`.

4. **Performing the Test**:
   - Call `double_value` with an immutable reference to `x` (using `&x`).
   - Store the returned value in the variable `doubled`.

5. **Assertion**:
   - Use `assert_eq!` to verify that `doubled` is equal to `20`.
   - This assertion checks that the `double_value` function correctly doubles the value of `x` and returns the result.

6. **Execution**:
   - This is a standard Rust test, so it will be executed along with other tests when you run `cargo test`.
   - The test passes if the assertion holds true, meaning that `double_value` correctly doubles the input value.

This test is a straightforward example of how immutable borrowing works in Rust. By passing a reference (`&x`) instead of the value itself, you allow `double_value` to use `x` without taking ownership of it, ensuring `x` remains unchanged and available for use after the function call.

```rust
#[test]
fn test_immutable_borrowing() {
    fn double_value(value: &i32) -> i32 {
        *value * 2
    }

    let x = 10;
    let doubled = double_value(&x); // Immutable borrow
    assert_eq!(doubled, 20); // Asserting the result
}
```

In this test:
- The `double_value` function now returns an `i32` which is double the value of the borrowed reference.
- We assert that the result of `double_value(&x)` is equal to `20`, which is `10 * 2`.
- This test not only demonstrates immutable borrowing but also includes an assertion to confirm that the borrowed data is accessed and used correctly.