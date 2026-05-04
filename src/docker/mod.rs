use std::process::Command;

pub fn run_build(path: &str) -> std::process::Output {
    Command::new("docker")
        .arg("build")
        .arg(path)
        .output()
        .expect("Failed to execute docker build")
}