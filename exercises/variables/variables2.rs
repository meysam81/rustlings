// variables2.rs
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a hint.


// use std::cmp::Ordering;


fn main() {
    let x = 10;
    // if x == 10 {
    //     println!("x is ten!");
    // } else {
    //     println!("x is not ten!");
    // }
    match x {
        // Ordering::Equal => println!("x is ten!"),
        10 => println!("x is ten!"),
        _ => println!("x is not ten!"),
    }
}
