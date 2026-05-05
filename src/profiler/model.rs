use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct BuildStep {
    pub content: String,
    pub is_cached: bool,
    pub duration_sec: f32,
    pub id: u32,
}

#[derive(Serialize, Debug)]
pub struct BuildResult {
    pub duration_ms: u128,
    pub steps: Vec<BuildStep>,
}
