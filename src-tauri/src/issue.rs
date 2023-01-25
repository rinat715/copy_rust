use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct JiraResponse {
    expand: String,
    fields: Issue,
    id: String,
    key: String,
    #[serde(rename = "self")]
    self_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Issue {
    id: u64,
    key: String,
    summary: String,
    description: String,
}