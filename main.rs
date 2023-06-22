fn main() {
    println!("Hello world!");

    let greeting = "Hello world!";
    println!("{greeting}");
    //Tuple

    // function
    fn get_string() -> String {
        return String::from("Hello world!");
    }

    println!("{}", get_string())
}
