// 控制流演示

fn main() {
    // Loop 循环
    let mut i = 0;
    loop {
        i = i + 1;
        println!("loop, i={}", i);
        if i > 5 {
            break;
        }
    }

    // While 循环
    while i < 15 {
        i = i + 1;
        println!("while, i={}", i);
    }

    // For 循环
    for j in (i..20) {
        println!("for, j={}", j);
    }
    // For 循环，包含 25
    for j in (i..=25) {
        println!("for, j={}", j);
    }

    // IF 判定
    println!("i={}", i);
    if i == 15 {
        println!("i={}", i);
    }

    // 条件表达式作为结果
    let res = if i == 15 { 5 } else { 6 };
    println!("res={}", res);
}
