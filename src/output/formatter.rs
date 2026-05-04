use crate::profiler::model::BuildResult;

pub trait Formatter {
    fn format(&self, result: &BuildResult);
}

pub struct CliFormatter;

pub struct JsonFormatter;

impl Formatter for CliFormatter {
    fn format(&self, result: &BuildResult) {
        println!("=== BUILD SUMMARY ===");

        println!("Duration: {} ms", result.duration_ms);

        println!("\nSteps:");

        for step in &result.steps {
            print!("{}", step.content);
            if step.is_cached {
                print!(" (cached)");
            }
            println!();
        }
    }
}

impl Formatter for JsonFormatter {
    fn format(&self, result: &BuildResult) {
        println!("{}", serde_json::to_string(result).unwrap());
    }
}

pub fn chose_formatter(json: bool) -> Box<dyn Formatter> {
    if json == true {
        Box::new(JsonFormatter)
    } else {
        Box::new(CliFormatter)
    }
}
