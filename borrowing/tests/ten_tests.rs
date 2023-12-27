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

#[test]
#[ignore = "Example that would not compile"]
fn test_mutable_and_immutable_borrowing_conflict() {
    let mut x = 10;
    
    let _immutable_borrow = &x; // Immutable borrow, adds '_' to clear warnings
    let _mutable_borrow = &mut x; // Mutable borrow

    // Use both borrows here
    // ...

    // This code will not compile due to simultaneous mutable and immutable borrows
}

#[test]
fn test_lifetime_of_borrowed_reference() {
    fn double_value<'a>(value: &'a i32) -> i32 {
        *value * 2
    }

    let result = {
        let x = 42;
        double_value(&x)
    };

    assert_eq!(result, 84);

    // Any attempt to use x or a reference to x here would result in a compile-time error,
    // as x's scope has ended.
}

#[test]
fn test_struct_borrowing() {
    struct MyStruct {
        value: i32,
    }

    fn increment_field(value: &mut i32) {
        *value += 1;
    }

    let mut my_struct = MyStruct { value: 10 };
    increment_field(&mut my_struct.value);

    assert_eq!(my_struct.value, 11);
}

#[test]
fn test_borrowing_in_loops() {
    fn sum(values: &[i32]) -> i32 {
        values.iter().sum()
    }

    let numbers = vec![1, 2, 3, 4, 5];
    let mut total = 0;

    for _ in 0..numbers.len() {
        total += sum(&numbers);
    }

    assert_eq!(total, 5 * (1 + 2 + 3 + 4 + 5));
}

#[test]
#[ignore = "Example that would not compile"]
fn test_dangling_pointer_prevention() {
    struct Holder<'a> {
        _reference: &'a i32, // adds '_' to clear warnings
    }

    let _holder: Holder;
    {
        let x = 10;
        _holder = Holder { _reference: &x };
    } // x goes out of scope here

    // Accessing holder.reference here would be a dangling pointer,
    // but Rust's borrow checker will prevent this code from compiling.
}

#[test]
fn test_borrowing_slices() {
    fn sum_of_slice(slice: &[i32]) -> i32 {
        slice.iter().sum()
    }

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice = &numbers[2..5];

    let sum = sum_of_slice(slice);
    assert_eq!(sum, 3 + 4 + 5);
}

#[test]
fn test_function_returning_borrowed_reference() {
    struct Container {
        value: i32,
    }

    impl Container {
        fn get_value<'a>(&'a self) -> &'a i32 {
            &self.value
        }
    }

    let container = Container { value: 42 };
    let value_ref = container.get_value();

    println!("Address of container.value: {:p}", &container.value);
    println!("Address held by value_ref: {:p}", value_ref);

    assert_eq!(*value_ref, 42);
}
