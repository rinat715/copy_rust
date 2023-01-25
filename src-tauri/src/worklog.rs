use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorklogResponse {
    start_date: String,
    end_date: String,
    projects: Vec<Project>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Project {
    id: u64,
    name: String,
    key: String,
    issues: Vec<Issue>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Issue {
    id: u64,
    key: String,
    summary: String,
    work_logs: Vec<WorkLog>,
}


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct WorkLog {
    id: u64,
    work_start: u64,
    time_spent: u16,
    work_log_attributes: Vec<WorkLogAttribute>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct WorkLogAttribute {
    id: u64,
    attr_type_id: u8,
    attr_value: String,
}
