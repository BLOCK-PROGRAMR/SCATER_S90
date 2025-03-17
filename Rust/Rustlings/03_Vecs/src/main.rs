fn main() {
    println!("Hello, world!");
    // array_Vec();
    iterator_methods();
}

fn array_Vec(){
    let _arr=[2,3,4,5,6];
    let numbers:Vec<i32>=vec![2,3,4,5,6];
    let mut num:Vec<i32>=Vec::new();
    num.push(4);
    num.push(5);
    println!("{:?}",num);
    println!("{:?}",numbers);
}
fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
          output.push(element*2);;
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
   
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    input
        .iter()
        .map(|element| {
            element*2
        })
        .collect()//collect() is used to convert the iterator into a collection(array)
}

fn iterator_methods(){
    let mut numbers = vec![1, 2, 3];

    // Immutable iterator
    for num in numbers.iter() {
        println!("Immutable: {}", num);
    }

    // Mutable iterator
    for num in numbers.iter_mut() {
        *num *= 2; // Modify in-place
    }

    // Into iterator (takes ownership)
    for num in numbers.into_iter() {
        println!("Owned: {}", num);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
