fn main() {
    println!("Hello, world!");

    // integers
    let num: i32 = 200;
    let num2 = 400;

    // floats
    let x = 2.0; // f64
    let y: f32 = 3.0;

    // numeric operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    // boolean type

    let t = true; // type inferred
    let f: bool = false; // explicit type

    // character type
    let c = 'b';
    let z: char = 'D';
    let emoji = '😝';

    // tuple type
    let tuple: (i32, f64, u8) = (255, 6.4, 1);

    // tuple destructuring
    let (x, y, z) = tuple;

    println!("The value of x, y and z is: {}, {} and {}", x, y, z);

    let first_item = tuple.0;

    // Arrays
    let arr: [i32, 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    // fill array with the same value
    let arr = [10; 9]; // [9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
}
