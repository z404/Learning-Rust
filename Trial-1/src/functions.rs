//defining a constant variable
const MAX_NUM: i16 = 20;

//creating a structure
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

pub fn main() {
    //using constant variable
    for n in 1..MAX_NUM {
        println!("{}", n);
    }

    //making a tuple
    //unpacking works
    let tupl = (1, 2, 3, 4);
    println!("{}", tupl.2);

    //function call
    let a = print_to_number(7);
    println!("{}", a);

    //creating a code block
    //it has access to scope outside, but its variables are not available outside
    {
        let _y = 10;
        // println!("{} is available", y);
    }

    //shadowing helps make variables local in codeblocks
    let mut x = 10;
    {
        let x = 15;
        println!("value of x is {}", x)
    }
    println!("value of x is {}", x);

    //making a pointer
    {
        //this is in a code block because the println command borrows x as an immutable reference
        //while xr borrows it as a mutable referance at the same time
        //therefore, to stop the error, you put this in a code block
        let xr = &mut x;
        *xr += 1;
    }
    println!("value of x is {}", x);

    //creating a variable from a structure
    let mut bg = Color {
        red: 255,
        green: 70,
        blue: 115,
    };
    //mut variables are changable
    bg.green = 35;
    //they are accessable with the . syntax
    println!("{} {} {}", bg.red, bg.blue, bg.green);
}

//defining a function
fn print_to_number(num: u32) -> bool {
    for i in 1..num {
        println!("{}", i);
    }
    return (num % 2) == 0;
}
