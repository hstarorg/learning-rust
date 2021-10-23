// 定义方法

fn main() {
    let mut user = User {
        name: String::from("Jay"),
        age: 30,
        email: String::from(""),
    };
    println!("user={:#?}", user);
    // 修改，需要定义为 mut
    user.update_age(18);
    // 打印
    print!("after update:");
    User::print(&user);
   
}

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    email: String,
}

impl User {
    // 传入自身，实例方法
    fn update_age(&mut self, new_age: u8) {
        self.age = new_age;
    }
    // 关联函数，可以简单理解为静态方法
    fn print(user: &User) {
        println!("{:#?}", user);
    }
}
