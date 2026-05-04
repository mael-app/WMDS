use std::time::Instant;
use crate::{docker, parser};

pub fn run_profile(path: &str) {
    let start = Instant::now();

    let output = docker::run_build(path);

    let duration = start.elapsed();

    let logs = String::from_utf8_lossy(&output.stderr);

    let steps = parser::extract_steps(&logs);

    println!("=== BUILD SUMMARY ===");
    println!("Duration: {:?}", duration);

    println!("\nSteps:");
    for step in steps {
        println!("{}", step.content);
    }
}