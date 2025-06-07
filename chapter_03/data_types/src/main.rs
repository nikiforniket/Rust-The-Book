fn main () {
    // example from chapter 2 guessing game
     let guess : i32 = "42".parse().expect("Not a number!");
     println!("variable 1 : {guess}");
     let guess : u32 = "-kk".parse().expect("Not a number!");
     println!("variable 2 : {guess}");
     let guess = "42".parse().expect("Not a number!"); // If type not mentioned it raises error.
     println!("variable 3 : {guess}");
}