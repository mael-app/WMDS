pub mod formatter;

use crate::profiler::model::BuildResult;

pub fn print(result: BuildResult, json: bool) {
    let formatter = formatter::chose_formatter(json);
    formatter.format(&result);
}
