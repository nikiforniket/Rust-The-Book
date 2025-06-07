fn main () {
    // Boolean type size is 1 byte, ie true or false;
    let t = true;
    let t_typed : bool = true;
    println!("True boolean {t}, with type annotations {t_typed}");
    let f = false;
    let f_typed:bool = false;
    println!("False boolean {f}, with type annotations {f_typed}");
}
