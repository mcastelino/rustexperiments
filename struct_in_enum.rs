struct Pointer {
    x: i32,
    y: i32,
    z: i32,
}
enum Message {
    Quit,
    Value(i32),
    Point(i32, i32, i32),
    EPointer(Pointer),
    Pointer { x: i32, y: i32 },
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit\n"),
        Message::Value(r) => println!("Value: {}\n", r),
        Message::Point(x, ..) => println!("Point {}\n", x),
        Message::EPointer(p) => println!("Pointer {}\n", p.x),
        Message::Pointer { x: x, y: y } => println!("Pointer {}, {}\n", x, y),
    };
}

fn main() {
    let msg1 = Message::Value(100);
    let msg2 = Message::Point(77, 88, 99);
    let msg3 = Message::Quit;
    let msg4 = Message::EPointer(Pointer {
        x: 33,
        y: 44,
        z: 55,
    });
    let msg5 = Message::Pointer { x: 33, y: 44 };

    process_message(msg1);
    process_message(msg2);
    process_message(msg3);
    process_message(msg4);
    process_message(msg5);
}

