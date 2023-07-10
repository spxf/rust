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

    // =================== assert_eq ===================
    // assert_eq if fail will throw an error and cancel the program
    // assert_eq!(6, 5); this code will broke prod
    assert_eq!(5, 5);
    
    // array
    println!("Array ==================");
    let arr1 = [1,2,3,4, 65];
    let arr2: [i32; 20] = [0; 20];
    
    println!("Array 1 is {:?}", arr1);
    println!("Array 2 is {:#?}", arr2);

    println!("Array 1 length is {:?}", arr1.len());
    println!("Array 1 memory size is {}", mem::size_of_val(&arr1));
    
    // looping
    let looping_arr = [1,2,3,4];
    for i in 0..looping_arr.len() {
        println!("Index is {}", i);
        println!("Item is {}", looping_arr[i])
    }
    
    


    // =================== Slice ===================
    let slice1 : &[i8] = &[0, 1, 2,3,4];
    println!("Slice 1 length is {}", slice1.len());

    // match, like switch case
    fn ov_match() {
        let condition = 2;

        match condition {
            1 => println!("Overview match success, condition is 1"),
            2 => println!("Overview match success, condition is 2"),
            _ => println!("Overview match success, condition is not founded")
        }
    }
    ov_match();
    // Vector
    let mut numbers_vec:Vec<i32> = Vec::new();
    numbers_vec.push(10);
    numbers_vec.push(20);
    numbers_vec.push(30);
    println!("Vec numbers first elements is {}", numbers_vec[0]);

    // 

    // Function ===========
    fn get_greeting() {
        println!("Hello world!");
    }
    get_greeting();
    fn get_greeting_with_return() -> String {
        return String::from("Hello world!");
    }
    println!("Funtion end with return statement : {}", get_greeting_with_return());
    fn get_greeting_without_return(str: String) -> String {
        str
    }
    println!("Funtion can end without return statement : {}", get_greeting_without_return(String::from("Hello world")));

    // Struct
    struct Student {
        name: String;
    } 

    pub trait year_only {
        fn year
    }
}
