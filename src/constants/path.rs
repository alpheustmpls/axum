#![allow(dead_code)]

use std::path::PathBuf;

use workspace_root::tokio::get_workspace_root_async;

/// Get app root.
pub async fn get_app_root() -> PathBuf {
    get_workspace_root_async().await
}
