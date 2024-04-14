// enums1.rs
//
// No hints this time! ;)

// 20240414

#[derive(Debug)]
enum Message {
    Quit,
	Echo,
	Move,
	ChangeColor
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
