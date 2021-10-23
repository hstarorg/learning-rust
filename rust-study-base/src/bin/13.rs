// 特性：所有权（悬垂引用：悬垂指针是其指向的内存可能已经被分配给其它持有者）

fn main() {
    // 如下调用会报错，返回的引用指向的内存区域已经被释放了
    // let reference_to_nothing = dangle();
    let safe_str = safe();
    println!("safe_str={}", safe_str);
}

// 需要先注释，避免编译错误
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 返回字符串 s 的引用，s 离开函数就被释放，引用是无效的
// }

fn safe() -> String {
    let s = String::from("hello");
    s
}
