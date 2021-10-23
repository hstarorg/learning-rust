// 获取输入并输出

use std::io;

fn main() {
    println!("请输入一个数字：");
    // 定义一个可变变量
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("输入错误");

    println!("你的输入是 {}", num);
}
