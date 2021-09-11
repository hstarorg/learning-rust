use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn test_rust_cli_success() -> Result<(), Box<dyn std::error::Error>> {
    // 通过 bin 名称来实例化一个命令行应用
    let mut cmd = Command::cargo_bin("rust-cli")?;

    // 配置参数，如下三个参数的拼接结果是："required arg" "--arg1" "12" "-b" "44"
    cmd.arg("required arg");
    cmd.arg("--arg1").arg("12");
    cmd.arg("-b").arg("44");

    // 开始断言
    let result = cmd.assert();
    // 命令行要执行成功，通过控制台输出中需要包含 required arg
    result
        .success()
        .stdout(predicate::str::contains("required arg"));
    Ok(())
}

#[test]
fn test_rust_cli_failuer() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-cli")?;
    // 直接执行命令行会出错，同时错误信息需要包含 error 这段文本
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: The following required arguments were not provided",
    ));
    Ok(())
}
