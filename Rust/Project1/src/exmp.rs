struct MyBox<T> {
    value: T,
}
struct MyVault<G>{
    value:G,
}
//here i was declaring the struct MyBox and MyVault with a generic type and G respectively
impl<T> MyBox<T> {//use of impl in rust is to implement methods for a struct 
    pub fn new(val: T) -> Self {//Self is a type that refers to the struct itself
        MyBox { value: val }
    }
    pub fn get(&self) -> &T {//it is a reference variable and it is not owning the  value
        &self.value
    }
}
impl<G> MyVault<G>{
    pub fn new (val:G)->Self{
        MyVault{value:val}
    }
    pub fn get(self)->&G{
        &self.value
    }
}

fn main() {
    let box1 = MyBox::new(123);
    let value = box1.get();
    println!("Value is: {}", value);// Output: Value is: 123
    let box2=MyBox::new("Hello Nithi How are u ?");
    let value2=box2.get();
    println!("Value is: {}", value2);// Output: Value is: Hello Nithi How are u ?
    let num=90;
    let vault=MyVault::new(&num);
    let value3=vault.get();
    println!("Vault value is :{}",value3);
    println!("num value:{}",num);
}
