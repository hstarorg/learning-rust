// 数据类型定义

fn main() {
    // f64
    let n1 = 4.63;
    let n2: f32 = 4.63;
    let c1: char = '中';
    let b1 = n1 == n2;
    println!("{}, {}", b1, c1);

    // 定义字符串
    let s1: &str = "中文";
    println!("{} -> {}", c1, s1);

    // 不加 r 前缀，转义
    let s2: &str = "A\nB";
    // 加上 r 前缀，原样输出
    let s3: &str = r"A\nB";
    println!("{} -> {}", s2, s3);
}
