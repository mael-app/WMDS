use crate::profiler::model::BuildResult;

pub trait Formatter {
    fn format(&self, result: &BuildResult);
}

pub struct CliFormatter;

impl Formatter for CliFormatter {
    fn format(&self, result: &BuildResult) {
        println!("=== BUILD SUMMARY ===");

        println!("Duration: {} ms", result.duration_ms);

        println!("\nSteps:");

        for step in &result.steps {
            println!("{}", step.content);
        }
    }
}