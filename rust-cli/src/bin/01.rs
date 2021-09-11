use std::env;

fn main() {
    // 取出命令行参数
    let mut args = env::args();
    let arg1 = args.nth(1).expect("请提供第一个参数");
    println!("第一个参数是：{}", arg1);
}
