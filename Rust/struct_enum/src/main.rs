
use std::io;


struct LpunNetwork {
    name: String,
    ip: String,
    port: u32,
    gateway: String,
    connected: bool,
}

// Implementation block for methods
impl LpunNetwork {
    fn new(name: &str, ip: &str, port: u32, gateway: &str, connected: bool) -> LpunNetwork {
        LpunNetwork {
            name: name.to_string(),
            ip: ip.to_string(),
            port,
            gateway: gateway.to_string(),
            connected,
        }
    }

    fn connect(&mut self) {
        self.connected = true;
    }

    fn disconnect(&mut self) {
        self.connected = false;
    }

    fn is_connected(&self) -> bool {
        self.connected
    }
}

fn main() {
    // println!("Hello, world!");
    
    // let mut lpun_network = LpunNetwork::new("LPUN", "0.8.8.8", 8080, "0.8.21", false);

    // println!("Name: {}", lpun_network.name);
    // println!("IP: {}", lpun_network.ip);
    // println!("Port: {}", lpun_network.port);
    // println!("Gateway: {}", lpun_network.gateway);
    
    // println!("Connected: {}", lpun_network.is_connected());
    // lpun_network.connect();
    // println!("Connected: {}", lpun_network.is_connected());
    // let result = divide(10.0, 0.0);
    // match result {
    //     Some(value) => println!("Result: {}", value),
    //     None => println!("Cannot divide by zero!"),
    // }
    // let _value= Currency_Value(Currency::data("Nithin".to_string()));
    //  println!("{}",_value);
    calling_messageenum();
    
    loop{
        currency();
        println!("Do you want to play again? (y/n)");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        if choice.trim().to_lowercase() != "y" {
            println!("Thanks for playing!");
            break;
        }
    }
    
    // let _value= Currency_Value(Currency::data(inputtake));
}


//enum for same way how ?

enum IpAddr{

    V4(u8,u8,u8,u8),
    V6(String),

}
enum Currency{
    BTC,
    ETH,
    XRP,
    LTC,
    USDT,
    data(String)
}

fn _enum() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

}

fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None // No valid result
    } else {
        Some(a / b) // Return result
    }
}

fn Currency_Value(coin:Currency)->f64{
    match coin{
        Currency::BTC=> 100.0,
        Currency::ETH=> 200.0,
        Currency::XRP=> 300.0,
        Currency::LTC=> 400.0,
        Currency::USDT=> 500.0,
        Currency::data(value)=> {
            println!("{}",value);
            0.0
        },
    }
}
fn currency(){
    println!("Enter the currency");
     let mut inputtake=String::new();
     io::stdin()
        .read_line(&mut inputtake)
        .expect("Failed to read line");
    let inputtake=inputtake.trim();
    println!("{}",inputtake);
   if(inputtake=="BTC" || inputtake=="btc"){
        println!("BTC");
        let _value= Currency_Value(Currency::BTC);
        println!("{}",_value);
    }
    else if(inputtake=="ETH" || inputtake=="eth"){
        println!("ETH");
        let _value= Currency_Value(Currency::ETH);
        println!("{}",_value);
    }
    else if(inputtake=="XRP" || inputtake=="xrp"){
        println!("XRP");
        let _value= Currency_Value(Currency::XRP);
        println!("{}",_value);
    }
    else if(inputtake=="LTC"    || inputtake=="ltc"){
        println!("LTC");
        let _value= Currency_Value(Currency::LTC);
        println!("{}",_value);
    }
    else if(inputtake=="USDT"   || inputtake=="usdt"){
        println!("USDT");
        let _value= Currency_Value(Currency::USDT);
        println!("{}",_value);
    }
    else{
        println!("data");
        let _value= Currency_Value(Currency::data(inputtake.to_string()));
        println!("{}",_value);
    }
}

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Resize { width: u32, height: u32 },
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}
fn calling_messageenum(){
    let messages = [
        Message::Resize { width: 10, height: 30 },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}

