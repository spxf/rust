use std::mem;

mod s_vector;

use s_vector::s_vector;

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

    println!("One million test is written as {}", 1_000_000u32);

    // =================== assert_eq ===================
    // assert_eq if fail will throw an error and cancel the program
    // assert_eq!(6, 5); this code will broke prod
    assert_eq!(5, 5);

    // array
    println!("Array ==================");
    let arr1 = [1, 2, 3, 4, 65];
    let arr2: [i32; 20] = [0; 20];

    println!("Array 1 is {:?}", arr1);
    println!("Array 2 is {:#?}", arr2);

    println!("Array 1 length is {:?}", arr1.len());
    println!("Array 1 memory size is {}", mem::size_of_val(&arr1));

    // looping
    let looping_arr = [1, 2, 3, 4];
    for i in 0..looping_arr.len() {
        println!("Index is {}", i);
        println!("Item is {}", looping_arr[i])
    }

    // Code unreachable allowed
    #[allow(unreachable_code)]
    fn loop_with_unreachable_code() {
        let outer_loop_max_length: i8 = 4;
        let mut outer_count: i8 = 0;
        'outer: loop {
            outer_count += 1;
            if outer_count > outer_loop_max_length {
                break;
            }
            let mut inner_count: i8 = 0;
            loop {
                inner_count += 1;
                if inner_count > outer_loop_max_length {
                    println!("Code broken in inner loop");
                    break 'outer;
                }
            }
            println!("This point will never be reached");
        }
    }
    loop_with_unreachable_code();

    // Vector
    s_vector();

    // =================== Slice ===================
    let slice1: &[i8] = &[0, 1, 2, 3, 4];
    println!("Slice 1 length is {}", slice1.len());

    // match, like switch case
    fn ov_match() {
        let condition = 2;

        match condition {
            1 => println!("Overview match success, condition is 1"),
            2 => println!("Overview match success, condition is 2"),
            _ => println!("Overview match success, condition is not founded"),
        }
    }
    ov_match();

    // Function ===========
    fn get_greeting() {
        println!("Hello world!");
    }
    get_greeting();
    fn get_greeting_with_return() -> String {
        return String::from("Hello world!");
    }
    println!(
        "Funtion end with return statement : {}",
        get_greeting_with_return()
    );
    fn get_greeting_without_return(str: String) -> String {
        str
    }
    println!(
        "Funtion can end without return statement : {}",
        get_greeting_without_return(String::from("Hello world"))
    );

    // Struct, trait .......
    struct Student {
        subject_name: String,
    }

    struct Programmer {
        programming_language_name: String,
    }

    pub trait Learnable {
        fn learn(&self) -> String;
    }

    impl Learnable for Student {
        fn learn(&self) -> String {
            self.subject_name.clone()
        }
    }

    impl Learnable for Programmer {
        fn learn(&self) -> String {
            self.programming_language_name.clone()
        }
    }

    let student: Student = Student {
        subject_name: String::from("Math"),
    };

    let programmer: Programmer = Programmer {
        programming_language_name: String::from("Rust"),
    };

    println!("{:#?}", student.learn());
    println!("{:#?}", programmer.learn());
}
