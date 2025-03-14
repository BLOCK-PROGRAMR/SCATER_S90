fn main() {
    //moving ownership
    //Heap-allocated types like String and Vec<T> do not implement Copy. If we were to try to use the value after moving it, we would get a compile-time error.
    // let n1 = String::from("Hello nithin how are u ?");
    // // let n2 = n1;
    // let n2 = n1.clone();
    // println!("{}", n2);
    // println!("{}", n1);
    // let m1 = 5;
    // let m2 = m1;
    // println!("{}", m2);
    // println!("{}", m1); //here error is not coming
    //                     //borrowing
    // let s1 = String::from("Hello");
    // let len = change(&s1);
    // println!("The length of '{}' is {}.", s1, len);
    // //mutable borrowing
    // let mut s2 = String::from("Hello");
    // change_mut(&mut s2);

    // println!("{}", s2);

    // checkEffect();
    // let _dangle = dangle();
    // println!("{}", _dangle);

    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    println!("{}", s);
    s.clear();
    println!("helllo world nithin{}", s);
}
pub fn change(s: &String) -> usize {
    // s.push_str("world");
    println!("this is an example of borrowing{}", s);
    let len = s.len();
    len
}
fn change_mut(s: &mut String) {
    //using reference to mutable the string value one time only
    println!("this string is mutable{}", s);
    let r1 = &s;
    // let r2 = &mut s;
    println!("{}", r1);
    // println!("{}", r2);
    s.push_str("world");
    s.push_str("nithin");
    println!("{}", s);
}

fn checkEffect() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    println!("{}", r1);
    let r2 = &mut s;

    println!("{}", r2);
}
fn dangle() -> String {
    // ❌ ERROR: Returning a reference to a local variable
    let s = String::from("Hello Nithinkumar");
    s // ❌ Borrowing `s`, which will be dropped after function ends
}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("{:?}", bytes);
    //if i use enumerate then i will get the index and value like a tuple

    for (i, &item) in bytes.iter().enumerate() {
        println!("{},{}", i, item);
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
