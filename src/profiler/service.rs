use std::time::Instant;
use crate::{docker, parser};
use crate::profiler::model::BuildResult;

pub fn run_profile(path: String) -> BuildResult {
    let start = Instant::now();

    let output = docker::run_build(&path);

    let duration_ms = start.elapsed().as_millis();

    let logs = String::from_utf8_lossy(&output.stderr);

    let steps = parser::extract_steps(&logs);

    BuildResult {
        duration_ms,
        steps,
    }
}