// use color_backtrace::install;
// use colored::*;
// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

// fn main() {
//     loop {
//         guessing_game();

//         println!("Do you want to play again? (y/n)");

//         let mut choice = String::new();
//         io::stdin()
//             .read_line(&mut choice)
//             .expect("Failed to read line");

//         if choice.trim().to_lowercase() != "y" {
//             println!("Thanks for playing!");
//             break;
//         }
//     }
// }

// fn guessing_game() {
//     let secret_number = rand::thread_rng().gen_range(1..=100); //inclusive range
//     println!("Guess the number between 1 and 100!");

//     loop {
//         println!("Please input your guess.");

//         let mut guess = String::new();
//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("{}", "Invalid input! Please enter a number.".red());
//                 continue;
//             }
//         };

//         println!("You guessed: {}", guess);

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("{}", "Too small!".red()),
//             Ordering::Greater => println!("{}", "Too big!".red()),
//             Ordering::Equal => {
//                 println!("{}", "You win!".green());
//                 break;
//             }
//         }
//     }
// }

mod math;

fn main() {
    let result = math::add(5, 3);
    println!("5 + 3 = {}", result);
}
