struct ColorRegularStruct {
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    let mut vec = Vec::new();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    // let vec = fill_vec(vec);

    // for x in &vec {
    //     println!("{}", x);
    // }
//  let data="Hii helooo how are you".to_string();
//     get_char(&data);
//     string_uppercase(data);
//     // println!("{}",data);

  // Example instantiations
  let _color1 = ColorRegularStruct { red: 255, green: 0, blue: 0 };
  let _color2 = ColorTupleStruct(255, 0, 0);
  let unit = UnitStruct;

  println!("{:?}", unit);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {

    vec.push(88);

    vec
}


fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

fn string_uppercase( data: String) {
   let  data = data.to_uppercase();

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];//Ownership of vec0 is moved to fill_vec
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];
        assert_eq!(vec0, [22, 44, 66]);
        let vec1 = fill_vec(vec0);

        
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);
        let z = &mut x;
    //     y.push(42);this is wrong because y is borrowed as mutable
        z.push(13);
        assert_eq!(x, [42, 13]);
    }


    #[test]
    fn regular_structs() {
        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        let unit_struct = UnitStruct;

        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
