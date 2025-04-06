use std::collections::HashMap; 

// #[derive(Debug)]
trait Area {
    fn area(&self) -> u32;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Area for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // let l1=(5,6); 
    //   let  (width,height)=l1;
    //     let rect=Rectangle{width,height};
    //    let value= rect.area();
    //    println!("Area of Rectangle{}",value);
    //    let config_max = Some((3,6));
    //    if let Some((max,min)) = config_max {
    //        println!("The maximum is configured to be {max}->{min}");
    //    }
    let l1 = hashmaps(); 
    println!("{:?}", l1); 
}

pub fn hashmaps() -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    hm.insert("hello".to_string(), 1);
    hm.insert("hii".to_string(), 2);
    hm.insert("how".to_string(), 3);
    hm 
}
