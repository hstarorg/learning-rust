// 枚举

fn main() {
    let sex = Sex::女;

    // 带参数的枚举
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("Hi Rust!"));
    m.execute();
    Message::static_fn();

    // Option<T> 标准库枚举
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!(
        "some_number={:?}, some_string={:?}, absent_number={:?}",
        some_number, some_string, absent_number
    );
}

enum Sex {
    男,
    女,
}

enum IpAddr {
    V4(String),
    V6(String),
}

// 乱七八糟的枚举
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    // 实例方法
    fn execute(&self) {
        println!("my god! self={:#?}", self);
    }
    // 也支持静态方法
    fn static_fn() {
        println!("I'm a static fn.");
    }
}
