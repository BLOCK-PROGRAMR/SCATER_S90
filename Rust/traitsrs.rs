
trait Area{
    fn area(&self)->u32;
}

struct Rectangle {
    width:u32,
    height:u32
}


 impl Area for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}   

fn main(){
    let l1=(5,6); 
  let  (width,height)=l1;
    let rect=Rectangle{width,height};
   let value= rect.area();
   println!("Area of Rectangle{}",value);
   let config_max = Some((3,6));
   if let Some((max,min)) = config_max {
       println!("The maximum is configured to be {max}->{min}");
   }


}





