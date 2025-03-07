fn main() {
    println!("Hello, world!");

    // primitive data types or a.k.a scalar data types

    // INTEGER
    // the interger are real numbers that can be positive, negative or zero

    // the integers can be signed and unsigned
    // signed integers are integers that can be positive or negative
    // unsigned integers are integers that can only be positive

    // signed int are i8, i16, i32, i64, i128, isize
    // unsigned int are u8, u16, u32, u64, u128, usize
    
    // the default integer type is i32
    // signed integers are the default integer type and the range are -2^31 to 2^31 - 1 for i32
    // range of unsigned integers are 0 to 2^32 - 1 for u32

    let a: i32 = -100;
    let b: u32 = 354;

    println!("Signed integer is {}", a);
    println!("Unsigned integer is {}", b);

    // Float
    // floating numbers are the numbers that are represented in fractions with decimals.
    // in rust the float data type are f32 and f64
    // the default float type is f64

    let c: f32 = 3.14;
    let d: f64 = 3.14159;

    println!("Float 32 is {}", c);
    println!("Float 64 is {}", d);

    // Boolean
    // boolean data type is a data type that can only have two values which are true and false

    let e: bool = true;
    let f: bool = false;

    println!("Boolean true is {}", e);
    println!("Boolean false is {}", f);

    // Character
    // character data type is a data type that is a single unicode character

    let g: char = 'a';

    println!("Character is {}", g);
}
