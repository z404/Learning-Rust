//Main function for rust to run on startup
//pub stands for public
pub fn main() {
    //defining variables
    //This variable is immutable. It cannot be changed once assigned
    let x = 45;

    //This variable is mutable. It can be used as a regular variable
    let mut y = 70;

    //This is a print statement
    println!("Hello!");
    //This is a print statement with a variable in it
    println!("The value of x is {}", x);

    //variable's value can be changed as usual
    y += 3;
    //print statement without newline
    println!("The value of y is {}", y);

    //Rust has datatypes that are actually sizes of variables
    //Types that are used are:
    // i32 (32 bit signed integer)
    // i64 (64 bit signed integer)
    // u32 (32 bit unsigned integer)
    // u64 (64 bit unsigned integer)
    // f32 (32 bit signed float)
    // f64 (64 bit signed float)

    //Underscore next to variable name removes "unused variable" warning
    let _a: f32 = -5.5;
    let b: u32 = 70;

    //if commands in rust
    if b > 65 {
        println!("If command!");
    } else if b == 70 {
        println!("Equal!!");
    } else {
        println!("Else command!!!");
    }
}
