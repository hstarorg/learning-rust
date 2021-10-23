// 变量定义

fn main() {
    // 定义常量
    const PI: f64 = 3.14;

    // 定义可变变量
    let mut num = 5;
    // 输出 PI=3.14,num=5
    println!("PI={},num={}", PI, num);

    num = 10;
    // 输出 num=10
    println!("num={}", num);

    // 定义不可变变量
    let num2 = 5;
    // 会报错：cannot assign twice to immutable variable
    // num2 = 6; // 注释掉才能编译通过
    println!("num2={}", num2);

    // 变量隐藏，通过 let 关键词复用变量名
    let num2 = 10;
    // 输出 num2=10
    println!("num2={}", num2);

    // 隐藏可变变量 num，之后 num 将不可变了
    let num = 20;
    // num = 222; // 需要注释掉才能编译通过，证明是一个全新的不可变变量
    // 输出 num=20
    println!("num={}", num);

    // 常量不可以被隐藏
    // let PI = 11; // 需要注释掉才能编译通过
}
