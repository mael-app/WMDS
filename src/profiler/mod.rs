use crate::docker;
use std::time::Instant;

pub fn run_profile(path: &str) {
    println!("DEBUG: entering run_profile");

    let start = Instant::now();

    let output = docker::run_build(path);

    let duration = start.elapsed();

    println!("DEBUG: got docker output");

    println!("Build duration: {:?}", duration);

    println!("STDERR:");

    println!("{}", String::from_utf8_lossy(&output.stderr));
}
