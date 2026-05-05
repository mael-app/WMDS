use crate::profiler::model::BuildStep;

pub fn extract_steps(logs: &str) -> Vec<BuildStep> {
    let mut steps: Vec<BuildStep> = Vec::new();
    let mut current_step: Option<usize> = None;

    for line in logs.lines() {
        if let Some(id) = extract_id(line) {
            if let Some(new_line) = get_str_without_id(line) {
                process_line_with_id(id, new_line, &mut steps, &mut current_step);
            }
        }
    }

    steps
}

fn process_line_with_id(
    id: u32,
    line: &str,
    steps: &mut Vec<BuildStep>,
    current_step: &mut Option<usize>,
) {
    if !is_current_step(id, *current_step, steps) {
        *current_step = Some(steps.len());
        steps.push(BuildStep {
            content: line.to_string(),
            id,
            is_cached: false,
            duration_sec: 0.0,
        });
    }
    if line.contains("CACHED") {
        if let Some(idx) = current_step {
            if let Some(step) = steps.get_mut(*idx) {
                step.is_cached = true;
            }
        }
    }
    if line.contains("DONE") {
        if let Some(idx) = current_step {
            if let Some(step) = steps.get_mut(*idx) {
                if let Some(duration_sec) = extract_duration_sec(line) {
                    step.duration_sec = duration_sec;
                }
            }
        }
    }
}

fn is_current_step(id: u32, current_step: Option<usize>, steps: &[BuildStep]) -> bool {
    match current_step {
        Some(idx) => steps.get(idx).map_or(false, |step| step.id == id),
        None => false,
    }
}

fn get_str_without_id(line: &str) -> Option<&str> {
    if let Some(pos) = line.find(' ') {
        let raw_id = &line[..=pos];
        return Option::from(line.trim_start_matches(raw_id));
    }

    None
}

fn extract_duration_sec(line: &str) -> Option<f32> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() >= 2 {
        let time_str = parts[1].trim_end_matches('s');
        return time_str.parse::<f32>().ok();
    }

    None
}

fn extract_id(line: &str) -> Option<u32> {
    if let Some(pos) = line.find(' ') {
        let raw_id = &line[..pos];
        return raw_id.trim_start_matches('#').parse::<u32>().ok();
    }

    None
}
