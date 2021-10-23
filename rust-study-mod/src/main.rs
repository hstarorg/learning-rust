// 引入本地项目中的三方库，需要写入 Cargo.toml
use core_crate::hello;

// 引入 src/lib.rs 中导出的内容
use rust_study_mod::dal::user_dal;
use rust_study_mod::hi;
use rust_study_mod::utils::log_util;

// 指定搜索路径：src/inner-lib.rs or src/inner-lib/mod.rs
mod inner_lib;
use crate::inner_lib::lib2;

fn main() {
    let user = hi::User {
        name: String::from("Jay"),
    };
    log_util::log(&user.name);
    hello::test();
    user_dal::get_user_info();
    inner_lib::hi();
    lib2::hi();
}
