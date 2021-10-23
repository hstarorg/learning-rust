// 结构体

fn main() {
    let user = User {
        name: String::from("Jay"),
        age: 30,
    };

    let user2 = User {
        name: String::from("John"),
        // 不会覆盖name，只会将 age 赋值过来
        ..user
    };
    println!("user.age = {}, user2.age={}", user.age, user2.age);

    // 给 User 增加注解 #[derive(Debug)] 后，就可以用如下方式打印了
    println!("user={:?}, user2={:#?}", user, user2);

    let p1 = Point(1, 1);
    let p2 = Point(4, 5);
    let point_distance = calculate_distance(p1, p2);
    println!("point_distance={}", point_distance);
}

// 结构体
#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

// 元组结构体
struct Point(u32, u32);

// 没有任何字段的类单元结构体
struct Empty {}

/**
 * 计算两个点的距离
 */
fn calculate_distance(p1: Point, p2: Point) -> u32 {
    let temp= (p2.0 - p1.0).pow(2) + (p2.1 - p1.1).pow(2);
    return (temp as f64).sqrt() as u32;
}
