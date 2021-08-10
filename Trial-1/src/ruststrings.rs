//creating struct for traits
struct Person {
    name: String,
    age: u8,
}

//creating trait
trait ToString {
    fn to_string(&self) -> String;
}

//adding trait to structure
impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("I am {} and i am {} years old", self.name, self.age);
    }
}

pub fn main() {
    let mut string = String::from("Hello I'm anish!");

    //length
    println!("length = {}", string.len());
    //is empty?
    println!("is empty? {}", string.is_empty());

    //splitting on whitespace
    for word in string.split_whitespace() {
        println!("{}", word);
    }

    //check substring
    println!(
        "Does the string contain anish? {}",
        string.contains("anish")
    );

    //append to string
    string.push_str(" Hello again!");
    println!("{}", string);

    //Traits
    let anish = Person {
        name: String::from("Anish"),
        age: 19,
    };
    println!("{}", anish.to_string());

    //vectors
    let _my_vector: Vec<i32> = Vec::new();
    let mut my_vector = vec![1, 2, 3, 4];
    println!("{}", my_vector[2]);
    my_vector.push(40);
    my_vector.remove(0);

    for number in my_vector.iter() {
        print!("{} ", number);
    }
}
