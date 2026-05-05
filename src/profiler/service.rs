use crate::profiler::model::BuildResult;
use crate::{docker, parser};
use std::borrow::Cow;
use std::time::Instant;

pub fn run_profile(path: String) -> BuildResult {
    let start = Instant::now();

    let output = docker::run_build(&path);

    let duration_ms = start.elapsed().as_millis();

    let logs = String::from_utf8_lossy(&output.stderr);

    if cfg!(debug_assertions) {
        print_raw_logs(&logs)
    }

    let steps = parser::extract_steps(&logs);

    BuildResult { duration_ms, steps }
}

fn print_raw_logs(logs: &Cow<str>) {
    println!("=== DEBUG ===");
    print!("{}", logs);
    println!("=============\n");
}
