use axum::Json;

use crate::error::AppError;
use crate::git_ops;
use crate::models::{
    CommitRequest, CommitResponse, DiffRequest, DiffResponse, ExtractRequest, ExtractResponse,
};

pub async fn extract_handler(
    Json(payload): Json<ExtractRequest>,
) -> Result<Json<ExtractResponse>, AppError> {
    let response = git_ops::extract_context(payload).await?;
    Ok(Json(response))
}

pub async fn commit_handler(
    Json(payload): Json<CommitRequest>,
) -> Result<Json<CommitResponse>, AppError> {
    let response = git_ops::create_in_memory_commit(payload).await?;
    Ok(Json(response))
}

pub async fn diff_handler(
    Json(payload): Json<DiffRequest>,
) -> Result<Json<DiffResponse>, AppError> {
    let response =
        git_ops::generate_diff(payload.repo_url, payload.base_sha, payload.target_sha).await?;
    Ok(Json(response))
}
