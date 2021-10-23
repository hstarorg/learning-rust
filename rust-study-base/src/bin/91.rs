// 错误&异常

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // 不可恢复的错误，panic
    let arr = vec![1, 2, 3];

    // 会直接 panic
    // println!("arr[10]={}", arr[10]);

    // 也可以直接触发 panic
    // panic!("Error");

    // 可恢复的错误，Result 类型来处理
    let f = File::open("non_file.txt");
    let f2 = match f {
        Ok(file) => file,
        Err(err) => {
            panic!("其他错误: {:?}", err);
            // if err.kind() == ErrorKind::NotFound {
            //     println!("未找到文件");
            //     return File::create("non_file.txt").unwrap();
            // } else {
            //     panic!("其他错误: {:?}", err);
            // }
        }
    };
}
