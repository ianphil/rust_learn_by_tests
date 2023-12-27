#[test]
fn test_immutable_borrowing() {
    fn double_value(value: &i32) -> i32 {
        *value * 2
    }

    let x = 10;
    let doubled = double_value(&x);
    assert_eq!(doubled, 20);
    assert_eq!(x, 10);
}

#[test]
fn test_mutable_borrowing() {
    fn increment(value: &mut i32) {
        *value += 1;
    }

    let mut x = 10;
    increment(&mut x);
    assert_eq!(x, 11);
}

#[test]
fn test_multiple_immutable_borrows() {
    fn square(value: &i32) -> i32 {
        value.pow(2)
    }

    let x = 4;
    let first_borrow = square(&x);
    let second_borrow = square(&x);

    assert_eq!(first_borrow, 16);
    assert_eq!(second_borrow, 16);
    assert_eq!(x, 4);
}