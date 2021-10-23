// 特性：所有权

fn main() {
    let s1 = "s1";
    let s1_copy = s1;
    println!("s1={}, s1_copy={}", s1, s1_copy);

    let s2 = String::from("s2");
    // 变量复制 -> 指针移动，原始指针会失效
    let s2_copy = s2;
    // 如下语句会失效，需要注释才能运行
    // println!("s2={}", s2); // error: value borrowed here after move
    // 手动指定要进行深拷贝
    let s2_new = s2_copy.clone();
    println!("s2_copy={}, s2_new={}", s2_copy, s2_new);

    // 引用移动
    let s3 = String::from("Hello");
    let s3_len = calculate_string_length(s3);
    // println!("s3={}, s3 len={}", s3, s3_len); // 会报错， s3 已经被释放
    // 借用（用完会把s4的所有权换回来）
    let s4 = String::from("Rust");
    let s4_len = calculate_string_length_pass_address(&s4);
    // 此时可以正常打印出来
    println!("s4={}, s4 len={}", s4, s4_len);
}

fn calculate_string_length(str: String) -> usize {
    str.len()
}

fn calculate_string_length_pass_address(str: &String) -> usize {
    return str.len();
}
