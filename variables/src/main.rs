fn main() {
    // Integers
    let w: i32 = 100_000;
    let x: i32 = 0xff; // You can represent numbers differently. Except for binary, it is only accepted as u8.
    let y: u8 = 255; // Value in bytes (beware of overflow, it will either panic or return to minimal values once past 255)
    let z: u8 = b'A'; // Value in bytes but literal(A = 65 in ASCII )
    // Floats
    let a: f32 = 3.0;
    let b: f64 = 2.0;
    // Booleans
    let t: bool = true;
    let f: bool = false;
    // Char and String
    let s: &str = "Hello";
    let c: char = 'c';

    // Operations
    let mut sum = w+w;
    let mut sub = w-w;
    let mut prod = w*2;
    let mut div = w/w;


    // Compound (you can store different types in a variable)
    let tup = ("Hello, world!", 40);
    let (helloworld, number) = tup; // you can assign variables for the vlaues in tup.
    let number = tup.1; // you can assign variables values by the index
    // Example:
    let codes = [200,404,500];
    let not_found = codes[1];
}
