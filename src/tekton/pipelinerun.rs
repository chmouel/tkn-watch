use std::io::Read;

use kube::{core::DynamicObject, Api};

use super::types::{Metadata, PipelineRun, PipelineRunStatus, TaskRunStatus};

pub async fn get(
    pipelinerun: Api<DynamicObject>,
    taskrun: Api<DynamicObject>,
    prname: &str,
) -> anyhow::Result<PipelineRun> {
    let obj = pipelinerun.get(prname).await?;

    let status = obj.data["status"]
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("PipelineRun has no status"))?;

    // is there a better way to convert that hashmap to string and back to the object we want?
    // there should be this looks like too many roundtrips
    let status_str = serde_json::to_string(&status)?;
    let status: PipelineRunStatus = serde_json::from_str(&status_str)?;

    let metadata_str = serde_json::to_string(&obj.metadata)?;
    let metadata: Metadata = serde_json::from_str(&metadata_str)?;

    if status.child_references.is_none() {
        return Ok(PipelineRun { metadata, status: None, child_status: None });
    }

    let mut task_status: Vec<TaskRunStatus> = vec![];
    for child in status.child_references.as_ref().unwrap() {
        let obj = taskrun.get(&child.name).await?;

        let child_status = obj.data["status"]
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("TaskRun has no status"))?;

        let child_status_str = serde_json::to_string(&child_status)?;
        let child_status: TaskRunStatus = serde_json::from_str(&child_status_str)?;
        task_status.push(child_status);
    }
    Ok(PipelineRun { metadata, status: Some(status), child_status: Some(task_status) })
}

// get all pipelineruns
pub async fn running(api: Api<DynamicObject>) -> anyhow::Result<Vec<String>> {
    let prs = api.list(&kube::api::ListParams::default()).await?;
    let mut prs = prs.items;
    prs.sort_by(|a, b| a.metadata.creation_timestamp.cmp(&b.metadata.creation_timestamp).reverse());
    let prs = prs
        .iter()
        .filter(|pr| {
            pr.metadata.name.is_some()
                && pr.data["status"].as_object().is_some()
                && pr.data["status"]["conditions"].as_array().is_some()
                && pr.data["status"]["conditions"][0].as_object().is_some()
                && pr.data["status"]["conditions"][0]["reason"] == "Running"
        })
        .map(|pr| pr.metadata.name.clone().unwrap())
        .collect::<Vec<_>>();
    Ok(prs)
}

pub fn from_json(filename: String) -> anyhow::Result<PipelineRun> {
    let mut file = std::fs::File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let pr: PipelineRun = serde_json::from_str(&contents)?;
    Ok(pr)
}
