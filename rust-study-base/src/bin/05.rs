// 猜数字游戏，猜正确为止，同时还要检测输入非数字场景

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let source = rand::thread_rng().gen_range(1..=100);
    println!("猜数字游戏！随机数字[0-100]已就绪");

    // 循环进行猜测~
    loop {
        println!("请输入你的猜测：");
        // 定义一个可变变量
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("输入错误");
        // 转换输入的字符串为数字
        // trim 是为了移除输入内容中的换行符
        let num: u16 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("你的输入是{}，不是数字，请重新输入", num.trim());
                // 重新输入
                continue;
            }
        };
        // 比较两个数字
        match num.cmp(&source) {
            Ordering::Equal => {
                println!("恭喜，猜测正确！");
                // 退出循环
                break;
            }
            Ordering::Greater => {
                println!("你猜得太大了！");
            }
            Ordering::Less => println!("你猜得太小了！"),
        }
    }
}
