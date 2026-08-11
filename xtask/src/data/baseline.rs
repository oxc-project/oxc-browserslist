use std::process::Command;

use anyhow::{Context, Result, ensure};
use serde::Deserialize;

use crate::utils::root;

#[derive(Deserialize)]
pub struct TimelineEvent {
    pub date: String,
    pub browsers: Vec<TimelineBrowserVersion>,
}

#[derive(Deserialize)]
pub struct TimelineBrowserVersion {
    pub browser: String,
    pub version: String,
}

pub fn load() -> Result<Vec<TimelineEvent>> {
    let script = r#"
const { getTimeline } = require('baseline-browser-mapping');
const timeline = getTimeline({
  listAllBrowsers: true,
  includeDownstreamBrowsers: true,
  includeKaiOS: true,
});
process.stdout.write(JSON.stringify(timeline));
"#;
    let output = Command::new("node")
        .args(["-e", script])
        .current_dir(root())
        .output()
        .context("run baseline-browser-mapping")?;
    ensure!(
        output.status.success(),
        "baseline-browser-mapping failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).context("parse baseline-browser-mapping timeline")
}
