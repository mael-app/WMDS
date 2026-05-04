use serde::Serialize;

#[derive(Serialize)]
pub struct BuildStep {
    pub content: String,
    pub is_cached: bool,
}

#[derive(Serialize)]
pub struct BuildResult {
    pub duration_ms: u128,
    pub steps: Vec<BuildStep>,
}
