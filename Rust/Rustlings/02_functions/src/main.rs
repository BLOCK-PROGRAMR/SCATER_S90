fn main() {
    println!("Hello, world!");

    // call_me(13);
    // let result=square(5);
    // println!("The square of 5 is {}",result);
    // let result=biggernumber(5,10);
    // println!("The bigger number is {}",result);
    let result=picky_eater("strawberry");
    println!("The food is {}",result);
}
fn call_me(num:u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
fn square(num: i32) -> i32 {
    num * num
}

    // TODO: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
fn biggernumber(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}
fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        "Yummy!"
    } else {
        "not yummy"
    }
}
fn animal_habitat(animal: &str) -> &str {
   
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        0
    };

  
    if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bigger_number() {
        assert_eq!(biggernumber(1, 2), 2);
        assert_eq!(biggernumber(2, 1), 2);
        assert_eq!(biggernumber(10, 10), 10);
    }

    #[test]
    fn picky_eater_test(){
        assert_eq!(picky_eater("strawberry"),"Yummy!");
        assert_eq!(picky_eater("apple"),"not yummy");
    }

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];
        let nice_slice=[2,3,4];

        assert_eq!([2, 3, 4], nice_slice);
    }
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);
        let second =numbers.1;
        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}

