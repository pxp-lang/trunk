use std::path::PathBuf;

pub fn cwd() -> PathBuf {
    std::env::current_dir().expect("Failed to get the current working directory.")
}

pub fn exec(command: &str, args: &[&str]) -> std::io::Result<std::process::Output> {
    std::process::Command::new(command)
        .args(args)
        .output()
}