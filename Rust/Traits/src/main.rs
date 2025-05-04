fn main() {
    let flower=Flower{
        dominant_color_gene:String::from("white"),
        recessive_color_gene:String::from("red"),
    };
    let bird=Bird{
        plumage_color:String::from("blue"),
    };
    // describe_color(&flower);
    // describe_color(&bird);
    let s1=String::from("hello");
    let s2=String::from("world");
    ownership(&s1,&s2);//clone means copy the value to the function 
    println!("s1 is {}",s1);//s1 is still valid
}

trait Color{    //interface 
    fn get_color(&self)->String;
}
struct Flower{
    dominant_color_gene:String,
    recessive_color_gene:String,
}
struct Bird{
    plumage_color:String
}
impl Color for Flower{
    fn get_color(&self)->String{
        if(self.dominant_color_gene == "red" && self.recessive_color_gene == "white"){
            return String::from("red");
        }
        else if(self.dominant_color_gene == "white" && self.recessive_color_gene == "red"){
            return String::from("white");
        }
        else{
            return String::from("unknown color");
        } 
    }
}
impl Color for Bird{
    fn get_color(&self)->String{
        return self.plumage_color.clone();//clone means copy the value to       
    }
}
fn describe_color(item:&Color){
    println!("The color is {}",item.get_color());
}

fn ownership <'a> (s1:&'a String,s2:&'a String) ->&'a str{
    let num=4;
    if(num==1){
        return s1;
    }
    else if(num==2){
        return s2;
    }
    else{
        return "both were false";
    }
    return "all of them were falso";

}