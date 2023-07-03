use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PipelineRun {
    pub metadata: Metadata,
    pub status: Option<PipelineRunStatus>,
    pub child_status: Option<Vec<TaskRunStatus>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskRun {
    pub metadata: Metadata,
    pub status: Option<TaskRunStatus>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub name: String,
    pub generate_name: Option<String>,
    pub namespace: String,
    pub labels: Option<HashMap<String, String>>,
    pub annotations: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PipelineRunStatus {
    pub completion_time: Option<String>,
    pub conditions: Option<Vec<KnativeCondition>>,
    pub child_references: Option<Vec<ChildReference>>,
    pub start_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChildReference {
    pub pipeline_task_name: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PipelineRunTaskRunStatus {
    pub pipeline_task_name: String,
    #[serde(flatten)]
    pub status: HashMap<String, TaskRunStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TaskRunStatus {
    pub completion_time: Option<String>,
    pub start_time: String,
    pub conditions: Option<Vec<KnativeCondition>>,
    pub steps: Option<Vec<StepStatus>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StepStatus {
    pub name: String,
    pub terminated: Option<StepStatusTerminated>,
    pub running: Option<StepStatusRunning>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StepStatusTerminated {
    pub reason: String,
    pub exit_code: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StepStatusRunning {
    pub started_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KnativeCondition {
    pub last_transition_time: String,
    pub message: String,
    pub reason: String,
    pub status: String,
}
