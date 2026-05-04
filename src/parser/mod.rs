use crate::profiler::model::BuildStep;

pub fn extract_steps(logs: &str) -> Vec<BuildStep> {
    let mut steps = Vec::new();

    for line in logs.lines() {
        if line.contains("RUN") {
            steps.push(BuildStep {
                content: line.to_string(),
            });
        }
    }

    steps
}
