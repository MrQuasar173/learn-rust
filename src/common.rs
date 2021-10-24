pub fn common() {

    // Variables and Mutability
    let mut x = 5;
    println!("The value of X is {}", x);
    x = 6;
    println!("The value of X is {}", x);

    // constants
    const PLANCK: f32 = 6.62607004;

    // Shadowing
    let _y = 8;
    let _y: i64 = 0x050c09; // You can use base10, hex, octal, binary, byte as literal for defining an int

    // I'm not going to write a out all of the scalar types and their examples cause that's simple
    // and just the exact same thing as (most) other languages
    // There's all the same math ops too, including %

    let _tuple = ("5K race PR", 18 /* minutes */, 18 /* seconds */);
    





}