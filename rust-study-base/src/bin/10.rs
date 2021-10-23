// 函数

fn main() {
    fn1();
    fn2(45);
    let res3 = fn3(3);
    println!("res3={}", res3);
}

// 无参数无返回值
fn fn1() {
    println!("call fn1");
}

// 有参数无返回值
fn fn2(num: i32) {
    println!("call fn2, arg num={}", num);
}

// 有参数有返回值
fn fn3(num: i32) -> i64 {
    println!("call fn3");
    return (num * 2).into();
}
