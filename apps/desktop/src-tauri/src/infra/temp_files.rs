//! Job-scoped temporary working copies with restrictive permissions.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use redact_domain::{AppError, ErrorCode};

const PREFIX: &str = "redact-job-";

#[allow(dead_code)] // Wired in US1/US7 export & cleanup paths
pub struct TempWorkingCopy {
    pub root: PathBuf,
}

#[allow(dead_code)]
impl TempWorkingCopy {
    pub fn create(base: &Path, job_id: &str) -> Result<Self, AppError> {
        let root = base.join(format!("{PREFIX}{job_id}"));
        fs::create_dir_all(&root).map_err(|e| {
            AppError::new(ErrorCode::Internal, format!("temp dir create failed: {e}"))
        })?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = fs::Permissions::from_mode(0o700);
            fs::set_permissions(&root, perms).map_err(|e| {
                AppError::new(ErrorCode::Internal, format!("temp chmod failed: {e}"))
            })?;
        }
        Ok(Self { root })
    }

    pub fn cleanup(self) -> Result<(), AppError> {
        if self.root.exists() {
            fs::remove_dir_all(&self.root).map_err(|e| {
                AppError::new(ErrorCode::Internal, format!("temp cleanup failed: {e}"))
            })?;
        }
        Ok(())
    }
}

/// Remove orphaned job dirs older than `max_age_secs` at startup.
pub fn sweep_orphans(base: &Path, max_age_secs: u64) -> Result<usize, AppError> {
    if !base.exists() {
        return Ok(0);
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let mut removed = 0usize;
    for entry in fs::read_dir(base)
        .map_err(|e| AppError::new(ErrorCode::Internal, format!("temp sweep read: {e}")))?
    {
        let entry = entry
            .map_err(|e| AppError::new(ErrorCode::Internal, format!("temp sweep entry: {e}")))?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if !name.starts_with(PREFIX) {
            continue;
        }
        let meta = entry.metadata().map_err(|e| {
            AppError::new(ErrorCode::Internal, format!("temp sweep meta: {e}"))
        })?;
        let modified = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        if now.saturating_sub(modified) >= max_age_secs {
            let _ = fs::remove_dir_all(entry.path());
            removed += 1;
        }
    }
    Ok(removed)
}
