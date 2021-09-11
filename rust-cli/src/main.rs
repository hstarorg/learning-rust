// use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Ho",
    version = "0.0.1",
    author = "Jay(hm910705@163.com)",
    about = "这是一个用 rust 编写的命令行工具 Demo"
)]
struct CommandLineOpt {
    /// 这是参数1，用于xxx
    #[structopt(short, long, default_value = "12")]
    arg1: u32,

    /// 这是参数2，别名 -b，用于 xxx
    #[structopt(short = "b", long = "arg2", default_value = "42")]
    arg2: u32,

    /// 这是参数3，必填，用于 xxx
    #[structopt(required = true)]
    arg3: String,
}

fn main() {
    let opt = CommandLineOpt::from_args();
    println!("{:?}", opt);
}
