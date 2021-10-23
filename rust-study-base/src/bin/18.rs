// 模式匹配

fn main() {
    let q = Q::Q1;
    let res = get_value(q);
    println!("res={}", res);
    let res2 = get_value(Q::Q4(666));
    println!("res2={}", res2);

    // 通配符
    judge_q4(Q::Q1);
    judge_q4(Q::Q3);

    // 简洁控制流（可以理解为是 match 的语法糖）
    let v = Some(4);
    if let Some(5) = v {
        println!("v=5");
    } else {
        println!("v!=5");
    }
    // 等价于
    match v {
        Some(5) => println!("v=5"),
        _ => println!("v!=5"),
    }
}

enum Q {
    Q1,
    Q2,
    Q3,
    // 绑定值
    Q4(u32),
}

fn get_value(q: Q) -> u32 {
    match q {
        Q::Q1 => 100_000,
        Q::Q2 => 200_000,
        Q::Q3 => {
            return 300_000;
        }
        Q::Q4(value) => value,
    }
}

fn judge_q4(q: Q) {
    match q {
        Q::Q1 => println!("Q1"),
        Q::Q2 => println!("Q2"),
        _ => println!("不是Q1、Q2"),
    }
}
