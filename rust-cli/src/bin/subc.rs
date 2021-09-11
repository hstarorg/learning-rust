use indicatif::ProgressBar;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Hos",
    version = "0.0.1",
    author = "Jay(hm910705@163.com)",
    about = "这是一个用 rust 编写的仿 Git 的子命令 Demo"
)]
enum CommandLineOpt {
    #[structopt(name = "clone", about = "Clone a repository into a new directory.")]
    Clone {
        /// 远端仓库地址
        #[structopt(required = true)]
        url: PathBuf,
    },
    #[structopt(
        name = "init",
        about = "Create an empty Git repository or reinitialize an existing one."
    )]
    Init {},
}

fn main() {
    let opt = CommandLineOpt::from_args();
    println!("{:?}", opt);
    let pb = ProgressBar::new(100);
    let dur = Duration::from_millis(5);
    for _ in 1..=100 {
        thread::sleep(dur);
        // pb.println(format!("[+] finished #{}", i));
        // 每次循环 + 1
        pb.inc(1);
    }
    pb.reset();
    for i in 1..=100 {
        thread::sleep(dur);
        // 直接设置
        pb.set_position(i);
    }
    pb.finish_with_message("done");
    println!("done!");
}
