pub fn main() {
    //Creating loop variable
    let mut b: u32 = 90;

    //Infinite loop
    loop {
        b += 1;
        if b == 95 {
            //Continue statement
            continue;
        }
        if b > 100 {
            //Break statement
            break;
        }
        //Continuous print statement
        println!("Value of b: {}", b);
    }

    //While loop
    //Break and continue also work
    while b <= 120 {
        //Mod operator
        if b % 5 == 0 {
            println!("The value of b is now {}", b);
        }
        b += 1;
    }

    //For loop
    //break and continue also work
    for i in 1..10 {
        print!("{}", i);
    }

    //Making an iterator
    let num = 1..5;
    for i in num {
        print!("{}", i);
    }

    //for adding new line
    println!();

    //Making an array
    let animals = vec!["Rabbit", "Dog", "Cot"];

    //.iter is used as animals array needs to be available after the for loop
    for i in animals.iter() {
        print!("{} ", i);
    }
    println!();

    //Enumerate adds an index number to the values and outputs iterator of tuples
    for (index, _animal) in animals.iter().enumerate() {
        print!("{} ", index);
    }
}
