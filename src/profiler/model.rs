pub struct BuildStep {
    pub content: String,
}

pub struct BuildResult {
    pub duration_ms: u128,
    pub steps: Vec<BuildStep>,
}
