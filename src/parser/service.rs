use crate::profiler::model::BuildStep;
use std::collections::HashMap;

pub fn extract_steps(logs: &str) -> Vec<BuildStep> {
    let mut temp_steps: HashMap<u32, BuildStep> = HashMap::new();

    for line in logs.lines() {
        if let Some(id) = extract_id(line) {
            process_line_with_id(id, line, &mut temp_steps);
        }
    }

    temp_steps.into_values().collect()
}

fn process_line_with_id(id: u32, line: &str, map: &mut HashMap<u32, BuildStep>) {
    if line.contains("RUN") {
        map.insert(
            id,
            BuildStep {
                content: line.to_string(),
                is_cached: line.contains("CACHED"),
            },
        );
    } else if line.contains("CACHED") {
        if let Some(step) = map.get_mut(&id) {
            step.is_cached = true;
        }
    }
}

fn extract_id(line: &str) -> Option<u32> {
    if let Some(pos) = line.find(' ') {
        let raw_id = &line[..pos];
        return raw_id.trim_start_matches('#').parse::<u32>().ok();
    }

    None
}
