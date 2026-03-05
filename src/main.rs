fn add(a: i32, b: i32) -> i32 {
    a + b
}

/*
This function performs subtraction of two integers. It takes two `i32` integers as parameters and returns their difference.
 */
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/*
The `main` function serves as the entry point of the program. It demonstrates the usage of the `add` and `subtract` functions by calling them with sample values and printing the results to the console.
 */
fn main() {
    let sum = add(5, 3);
    println!("The sum is: {}", sum);
    println!("The difference is: {}", subtract(5, 3));
}

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-1, 1), 0);
    assert_eq!(add(0, 0), 0);
}

#[test]
fn test_subtract() {
    assert_eq!(subtract(5, 3), 2);
    assert_eq!(subtract(3, 5), -2);
    assert_eq!(subtract(0, 0), 0);
}
