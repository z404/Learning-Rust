//creating a structure
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

//creating a structure to use impl with
struct Rectangle {
    width: u32,
    height: u32,
}

//impl keyword assigns functions and variables to an object of the given structure
impl Rectangle {
    //this function takes a self keyword so it can access its own members
    fn print_description(&self) {
        println!("Rectangle: {} x {}", self.width, self.height);
    }

    //function that returns stuff
    fn is_square(&self) -> bool {
        return self.width == self.height;
    }
}

pub fn main() {
    let blue = Color {
        red: 0,
        green: 0,
        blue: 255,
    };
    //if not passed by reference, the variable is moved out of this scope, and cannot be used after the statement
    print_color(&blue);

    //arrays in rust
    let mut numbers = [1, 2, 3, 4, 5];
    //array with datatypes
    let _data: [i32; 5] = [1, 2, 3, 4, 5];

    //works on index
    numbers[2] = 7;
    let _length_of_array = numbers.len();
    for n in numbers.iter() {
        print!("{} ", n);
    }
    println!();

    //creating an object of the struct
    let my_rect = Rectangle {
        width: 5,
        height: 5,
    };
    //calling the implied function on the object of the struct
    my_rect.print_description();
    println!("Rectangle is a square: {}", my_rect.is_square());
}

fn print_color(c: &Color) {
    println!("Color {} {} {}", c.red, c.green, c.blue);
}
