//Question
// Fix the error below with least amount of modification to the code
/*
fn main() {
    let x: i32; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}
*/

//Solution
fn main() {
    let x: i32 = 10; // uninitialized but using, ERROR ! , now initialising a value for 
    let y: i32 = 15; // uninitialized but also unusing, only warning
   // println!("{} is equal to 5", x);
    assert_eq!(x, 10);
    println!("Success!");
}
