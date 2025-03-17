const NUMBER:i32=23;


fn main() {
     
     // challenge1();
     // challenge2();
     challenge3();

}

fn challenge1() {
    let  number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);
    
    // TODO: Fix the compiler error by changing the line below without renaming the variable.
   let  number = 3;
   println!("Number plus two is: {}", number + 2);
}
fn challenge2(){

     println!("the number is {}",NUMBER);
}

fn challenge3(){
     // let mut x = 3;
     // println!("Number {x}");
 
     // x = 5; // Don't change this line
     // println!("Number {x}");
     // TODO: Change the line below to fix the compiler error.
     //new challenge
//let x: i32=90;
//challenge
let x:Option<i32>=Some(10);

if x == Some(10) {
    println!("x is ten!");
} else {
    println!("x is not ten!");
}


    println!("Number {}", x.unwrap()) ;
}