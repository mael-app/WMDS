pub mod formatter;

use crate::output::formatter::{CliFormatter, Formatter};
use crate::profiler::model::BuildResult;

pub fn print(result: BuildResult) {
    let formatter = CliFormatter;
    formatter.format(&result);
}
