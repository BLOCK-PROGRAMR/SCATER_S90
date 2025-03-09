// fn main() {
//     println!("Hello, world!");
// }

// use std::io;

// fn main() {
//     println!("Guess the number!");

//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("You guessed: {}", guess);
// }

// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }
// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

// fn main(){
//     let spaces = "   ";
//     let spaces = spaces.len();
//     println!("The value of spaces is: {spaces}");
// }

fn main(){
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {guess}");
}