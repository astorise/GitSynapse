use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ExtractRequest {
    pub repo_url: String,
    pub branch: String,
    pub paths: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct FileContent {
    pub path: String,
    pub content: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ExtractResponse {
    pub files: Vec<FileContent>,
}

#[derive(Debug, Deserialize)]
pub struct FileChange {
    pub path: String,
    pub new_content: String,
}

#[derive(Debug, Deserialize)]
pub struct CommitRequest {
    pub repo_url: String,
    pub branch: String,
    pub changes: Vec<FileChange>,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
}

#[derive(Debug, Serialize)]
pub struct CommitResponse {
    pub sha: String,
}

#[derive(Debug, Deserialize)]
pub struct DiffRequest {
    pub repo_url: String,
    pub base_sha: String,
    pub target_sha: String,
}

#[derive(Debug, Serialize)]
pub struct FileDiff {
    pub path: String,
    pub status: String,
    pub hunks: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DiffResponse {
    pub files: Vec<FileDiff>,
}
