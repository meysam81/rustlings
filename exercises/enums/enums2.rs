// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.


#[derive(Debug)]
enum Message {
    Move{x: i32, y: i32},
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

impl Message {
    
    fn call(&self) {
        use Message::*;
        // println!("{:?}", self);
        match self {
            Message::Echo(str_) => println!("Echo {str_:?}"),
            Message::Move{y, ..} => println!("Way to gooo to {y} vertically"),
            ChangeColor(r, g, b) => println!("ready to change color {r},{g},{b}?"),
            _ => println!("catch all: {self:?}"),
        }
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
