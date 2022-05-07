use std::io::Read;

use kube::{core::DynamicObject, Api};

use super::types::{Metadata, PipelineRun, PipelineRunStatus};

pub async fn get(api: Api<DynamicObject>, prname: &str) -> anyhow::Result<PipelineRun> {
    let obj = api.get(prname).await?;

    let status = obj.data["status"]
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("PipelineRun has no status"))?;

    // is there a better way to convert that hashmap to string and back to the object we want?
    // there should be this looks like too many roundtrips
    let status_str = serde_json::to_string(&status)?;
    let status: PipelineRunStatus = serde_json::from_str(&status_str)?;
    let metadata_str = serde_json::to_string(&obj.metadata)?;
    let metadata: Metadata = serde_json::from_str(&metadata_str)?;
    Ok(PipelineRun {
        metadata,
        status: Some(status),
    })
}

pub fn from_json(filename: String) -> anyhow::Result<PipelineRun> {
    let mut file = std::fs::File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let pr: PipelineRun = serde_json::from_str(&contents)?;
    Ok(pr)
}
