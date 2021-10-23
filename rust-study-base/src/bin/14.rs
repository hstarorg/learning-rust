// Slice 类型（无所有权的类型，字符串字面量就是slice）

fn main() {
    let str = String::from("hello rust");
    let part1 = &str[0..5];
    let part2 = &str[6..str.len()];
    println!("str={}, part1={}, part2={}", str, part1, part2);

    // 获取全部 str
    let str_copy = &str[..];
    println!("str={}, str_copy={}", str, str_copy);
}
