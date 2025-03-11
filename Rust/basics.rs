use std::io;

fn main() {
    //_tuple();
    //    _array();
    //  _array_index();
    // _dynamic_array();

    // let num: i32 = add_one(6);
    // println!("{}", num);
    // let num2: i32 = add_one_v2(7);
    // println!("{}", num2);
    for_loop();
}

fn _tuple() {
    let tup: (i32, f64, u8) = (600, 4.6, 2);
    println!("the value of tuple{:?}", tup); //to print complex tuples or ds u need to assign {:?} to the tuple
    println!("the value of tuple{},{},{}", tup.0, tup.1, tup.2); //to print individual values of tuple
    let (x, y, z) = tup;
    println!("the value of x is:{}", x);
    println!("the value of y is:{}", y);
    println!("the value of z is:{}", z);
}

fn _array() {
    //variable type  variable name: [data type;size]=[values];
    let a: [i64; 6] = [1, 2, 3, 4, 5, 6];
    println!("the value of array is:{:?}", a);
    println!(
        "the value of array is:{},{},{},{},{},{}",
        a[0], a[1], a[2], a[3], a[4], a[5]
    );
    let b: [i64; 6] = [1; 6];
    println!("the value of array is:{:?}", b);
    println!(
        "the value of array is:{},{},{},{},{},{}",
        b[0], b[1], b[2], b[3], b[4], b[5]
    );
}
fn _array_index() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    //syntax ===> varibale type mut variable name:datatype=new value;
    let mut index = String::new(); // u can take user index from command prompt

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn _dynamic_array() {
    let mut nums: Vec<i32> = Vec::new();
    nums.push(1);
    nums.push(2);
    nums.push(3);
    nums.push(7);
    println!("the value of nums is:{:?}", nums);
}

fn add_one(x: i32) -> i32 {
    x + 1 // Expression (returns a value)
}

fn add_one_v2(x: i32) -> i32 {
    //  x + 1 // Statement (no return value)
    return x + 1;
}

fn for_loop() {
    // for number in (1..4).rev() {
    //     println!("{number}!");
    // }
    for number in (1..4) {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
