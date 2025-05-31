fn main () {
    let x :i32 = 5;
    println!("The value of x 01 : {x}");
    let x :i32 = 6;
    {
        let x :i32 = 12;
        println!("Inside the secondary scope : {x}")
    }
    println!("The value of x 02 : {x}")
}