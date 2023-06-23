fn main() {
    println!("Hello world!");

    let greeting = "Hello world!";
    println!("{greeting}");
    //Tuple

    // match, like switch case
    fn ov_match() {
        let condition = 1;

        match condition {
            1 => println!("Overview match success, condition is {condition}"),
            _ => println!("Overview match success, condition is not founded")
        }
    }
    ov_match();


    // function
    fn get_string() -> String {
        return String::from("Hello world!");
    }

    println!("{}", get_string())
}
