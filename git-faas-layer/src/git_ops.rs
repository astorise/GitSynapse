use std::path::Path;

use git2::{
    build::RepoBuilder, Cred, DiffOptions, FetchOptions, PushOptions, RemoteCallbacks,
    Repository, Signature,
};

use crate::error::AppError;
use crate::models::{
    CommitRequest, CommitResponse, DiffResponse, ExtractRequest, ExtractResponse, FileContent,
    FileDiff,
};

fn make_fetch_options<'a>(token: Option<&'a str>) -> FetchOptions<'a> {
    let mut callbacks = RemoteCallbacks::new();
    if let Some(tok) = token {
        callbacks.credentials(move |_url, _username, _allowed| {
            Cred::userpass_plaintext("oauth2", tok)
        });
    }
    let mut fetch_opts = FetchOptions::new();
    fetch_opts.remote_callbacks(callbacks);
    fetch_opts
}

/// Clone the repository to a temp dir (shallow via depth=1) and return it.
fn shallow_clone(repo_url: &str, branch: &str) -> Result<(Repository, tempfile::TempDir), AppError> {
    let token = std::env::var("GITLAB_TOKEN").ok();
    let tmp = tempfile::tempdir().map_err(|e| AppError::Internal(e.to_string()))?;

    let mut fetch_opts = make_fetch_options(token.as_deref());
    fetch_opts.depth(1);

    let repo = RepoBuilder::new()
        .branch(branch)
        .fetch_options(fetch_opts)
        .clone(repo_url, tmp.path())
        .map_err(AppError::Git)?;

    Ok((repo, tmp))
}

// ---------------------------------------------------------------------------
// 2.1 extract_context
// ---------------------------------------------------------------------------

pub async fn extract_context(req: ExtractRequest) -> Result<ExtractResponse, AppError> {
    let req = req; // move into blocking closure
    tokio::task::spawn_blocking(move || {
        let (repo, _tmp) = shallow_clone(&req.repo_url, &req.branch)?;
        let head = repo.head().map_err(AppError::Git)?;
        let commit = head.peel_to_commit().map_err(AppError::Git)?;
        let tree = commit.tree().map_err(AppError::Git)?;

        let mut files = Vec::new();
        for path_str in &req.paths {
            let path = Path::new(path_str);
            match tree.get_path(path) {
                Ok(entry) => {
                    let obj = entry.to_object(&repo).map_err(AppError::Git)?;
                    match obj.as_blob() {
                        Some(blob) => {
                            let content = String::from_utf8_lossy(blob.content()).into_owned();
                            files.push(FileContent {
                                path: path_str.clone(),
                                content: Some(content),
                                error: None,
                            });
                        }
                        None => {
                            files.push(FileContent {
                                path: path_str.clone(),
                                content: None,
                                error: Some("entry is not a blob".to_string()),
                            });
                        }
                    }
                }
                Err(e) => {
                    files.push(FileContent {
                        path: path_str.clone(),
                        content: None,
                        error: Some(e.message().to_string()),
                    });
                }
            }
        }

        Ok(ExtractResponse { files })
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
}

// ---------------------------------------------------------------------------
// 2.2 create_in_memory_commit
// ---------------------------------------------------------------------------

pub async fn create_in_memory_commit(req: CommitRequest) -> Result<CommitResponse, AppError> {
    tokio::task::spawn_blocking(move || {
        let token = std::env::var("GITLAB_TOKEN").ok();
        let tmp = tempfile::tempdir().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut fetch_opts = make_fetch_options(token.as_deref());
        fetch_opts.depth(1);

        let repo = RepoBuilder::new()
            .branch(&req.branch)
            .fetch_options(fetch_opts)
            .clone(&req.repo_url, tmp.path())
            .map_err(AppError::Git)?;

        // Step 1: get parent commit
        let head = repo.head().map_err(AppError::Git)?;
        let parent_commit = head.peel_to_commit().map_err(AppError::Git)?;

        // Step 2: start a TreeBuilder from the existing tree
        let parent_tree = parent_commit.tree().map_err(AppError::Git)?;
        let mut tree_builder = repo
            .treebuilder(Some(&parent_tree))
            .map_err(AppError::Git)?;

        // Step 3: for each changed file, write blob and insert into tree
        for change in &req.changes {
            let blob_oid = repo
                .blob(change.new_content.as_bytes())
                .map_err(AppError::Git)?;
            tree_builder
                .insert(&change.path, blob_oid, 0o100644)
                .map_err(AppError::Git)?;
        }

        // Step 4: write the new tree
        let new_tree_oid = tree_builder.write().map_err(AppError::Git)?;
        let new_tree = repo.find_tree(new_tree_oid).map_err(AppError::Git)?;

        // Step 5: create signature
        let sig = Signature::now(&req.author_name, &req.author_email).map_err(AppError::Git)?;

        // Step 6: create commit object
        let commit_oid = repo
            .commit(
                Some(&format!("refs/heads/{}", req.branch)),
                &sig,
                &sig,
                &req.message,
                &new_tree,
                &[&parent_commit],
            )
            .map_err(AppError::Git)?;

        // Step 7: push to remote
        let mut remote = repo.find_remote("origin").map_err(AppError::Git)?;
        let refspec = format!(
            "refs/heads/{}:refs/heads/{}",
            req.branch, req.branch
        );

        let mut push_callbacks = RemoteCallbacks::new();
        if let Some(tok) = token {
            push_callbacks.credentials(move |_url, _username, _allowed| {
                Cred::userpass_plaintext("oauth2", &tok)
            });
        }

        let mut push_opts = PushOptions::new();
        push_opts.remote_callbacks(push_callbacks);

        remote
            .push(&[refspec.as_str()], Some(&mut push_opts))
            .map_err(AppError::Git)?;

        Ok(CommitResponse {
            sha: commit_oid.to_string(),
        })
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
}

// ---------------------------------------------------------------------------
// 2.3 generate_diff
// ---------------------------------------------------------------------------

pub async fn generate_diff(
    repo_url: String,
    base_sha: String,
    target_sha: String,
) -> Result<DiffResponse, AppError> {
    tokio::task::spawn_blocking(move || {
        let token = std::env::var("GITLAB_TOKEN").ok();
        let tmp = tempfile::tempdir().map_err(|e| AppError::Internal(e.to_string()))?;

        // We need a full clone to access arbitrary SHAs
        let mut fetch_opts = make_fetch_options(token.as_deref());
        let repo = RepoBuilder::new()
            .fetch_options(fetch_opts)
            .clone(&repo_url, tmp.path())
            .map_err(AppError::Git)?;

        let base_oid = repo
            .revparse_single(&base_sha)
            .map_err(AppError::Git)?
            .peel_to_commit()
            .map_err(AppError::Git)?
            .tree()
            .map_err(AppError::Git)?;

        let target_oid = repo
            .revparse_single(&target_sha)
            .map_err(AppError::Git)?
            .peel_to_commit()
            .map_err(AppError::Git)?
            .tree()
            .map_err(AppError::Git)?;

        let mut diff_opts = DiffOptions::new();
        let diff = repo
            .diff_tree_to_tree(Some(&base_oid), Some(&target_oid), Some(&mut diff_opts))
            .map_err(AppError::Git)?;

        let mut files: Vec<FileDiff> = Vec::new();

        diff.foreach(
            &mut |delta, _progress| {
                let path = delta
                    .new_file()
                    .path()
                    .or_else(|| delta.old_file().path())
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_default();
                let status = format!("{:?}", delta.status());
                files.push(FileDiff {
                    path,
                    status,
                    hunks: Vec::new(),
                });
                true
            },
            None,
            Some(&mut |delta, hunk| {
                let path = delta
                    .new_file()
                    .path()
                    .or_else(|| delta.old_file().path())
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_default();
                let header = String::from_utf8_lossy(hunk.header()).into_owned();
                if let Some(fd) = files.iter_mut().find(|f| f.path == path) {
                    fd.hunks.push(header);
                }
                true
            }),
            None,
        )
        .map_err(AppError::Git)?;

        Ok(DiffResponse { files })
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
}
