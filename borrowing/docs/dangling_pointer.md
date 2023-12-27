# test_dangling_pointer_prevention

The `test_dangling_pointer_prevention` test will illustrate how Rust's ownership and borrowing system prevents dangling pointers. A dangling pointer occurs when a pointer or reference points to memory that has been deallocated or moved, leading to undefined behavior. Rust's compiler checks prevent this from happening.

Test Outline:
1. Create a scenario where a pointer might typically become dangling in languages without Rust's safety guarantees.
2. Show that in Rust, the compiler will prevent this code from running, either by a compilation error or a borrow checker violation.

Here's a theoretical implementation of the test, as actual implementation would result in a compilation error:

```rust
#[test]
fn test_dangling_pointer_prevention() {
    struct Holder<'a> {
        reference: &'a i32,
    }

    let holder: Holder;
    {
        let x = 10;
        holder = Holder { reference: &x };
    } // 'x' goes out of scope here

    // Accessing 'holder.reference' here would be a dangling pointer,
    // but Rust's borrow checker will prevent this code from compiling.
}
```

In this test:
- We define a `Holder` struct that holds a reference to an `i32`.
- We create an `i32` value `x` inside a nested scope and initialize a `Holder` instance with a reference to `x`.
- `x` goes out of scope, which would normally make `holder.reference` a dangling pointer.
- Rust's borrow checker prevents this code from compiling because it recognizes that `holder.reference` would be pointing to deallocated memory.

This test is meant to demonstrate a compile-time error. It shows how Rust's lifetime and borrowing rules ensure memory safety by preventing access to invalid references. In practice, this code will not compile, which is exactly what we want to demonstrate.