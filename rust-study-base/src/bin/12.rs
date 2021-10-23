// 特性：所有权（引用、借用）

fn main() {
    // 引用（移动，所有权移动）
    let s3 = String::from("Hello");
    let s3_len = calculate_string_length(s3);
    // println!("s3={}, s3 len={}", s3, s3_len); // 会报错， s3 已经被释放

    // 借用（用完会把s4的所有权换回来）
    let s4 = String::from("Rust");
    let s4_len = calculate_string_length_pass_address(&s4);
    // 此时可以正常打印出来
    println!("s4={}, s4 len={}", s4, s4_len);

    // 可变借用（借出去的过程中可以被修改）
    let mut s5 = String::from("Hi");
    let s5_len = calculate_string_length_and_update(&mut s5, ", Rust");
    println!("s5={}, s5 len={}", s5, s5_len);
}

fn calculate_string_length(str: String) -> usize {
    str.len()
}

fn calculate_string_length_pass_address(str: &String) -> usize {
    return str.len();
}

fn calculate_string_length_and_update(str: &mut String, append_str: &str) -> usize {
    str.push_str(append_str);
    str.len()
}
