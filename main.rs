use std::mem;

fn main() {
    
    println!("Hello world!");

    let greeting = "Hello world!";
    println!("{greeting}");

    //primative
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
    
    println!("One million is written as {}", 1_000_000u32);
    
    // array
    println!("Array ==================");
    let arr1 = [1,2,3,4];
    let arr2: [i32; 20] = [0; 20];
    // let arr2: [i32; 500] = [0; 500];
    println!("Array 1 is {:?}", arr1);
    println!("Array 2 is {:?}", arr2);

    println!("Array 1 length is {:?}", arr1.len());
    println!("Array 1 memory size is {}", mem::size_of_val(&arr1));
    
    // looping
    for i in 0..arr2.len() {
        println!("Index is {}", i);
        // println!("Item is {}", arr2.get(i));
    }

    // match, like switch case
    fn ov_match() {
        let condition = 1;

        match condition {
            1 => println!("Overview match success, condition is {condition}"),
            _ => println!("Overview match success, condition is not founded")
        }
    }
    ov_match();


    // Function ===========
    fn get_greeting() {
        println!("Hello world!")
    }

    fn get_string() -> String {
        return String::from("Hello world!");
    }
    get_greeting();
    println!("{}", get_string())
    // Function ===========
}
