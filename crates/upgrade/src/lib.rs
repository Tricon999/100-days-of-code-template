
//! This crate provides the features of upgrading the maple executable.

mod github;

use crate::github::{asset_download_url, asset_name, retrieve_asset_size, retrieve_latest_release};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

/// This command is only invoked when user uses the prebuilt binary, more specifically, the
/// executable runs from `vim-clap/bin/maple`.
#[derive(Debug, Clone)]
pub struct Upgrade {
    /// Download if the local version mismatches the latest remote version.
    pub download: bool,
    /// Disable the downloading progress_bar
    pub no_progress_bar: bool,
}

impl Upgrade {
    pub fn new(download: bool, no_progress_bar: bool) -> Self {
        Self {
            download,
            no_progress_bar,
        }
    }

    pub async fn run(&self, local_tag: &str) -> std::io::Result<()> {
        println!("Retrieving the latest remote release info...");
        let latest_release = retrieve_latest_release().await?;
        let latest_tag = latest_release.tag_name;
        let latest_version = extract_remote_version_number(&latest_tag);
        let local_version = extract_local_version_number(local_tag);

        if latest_version != local_version {
            if self.download {
                println!("New maple release {latest_tag} is available, downloading...",);

                let temp_file = download_prebuilt_binary(&latest_tag, self.no_progress_bar).await?;

                // Only tries to upgrade if using the prebuilt binary, i.e., `bin/maple`.
                let bin_path = get_binary_path()?;

                // Move the downloaded binary to bin/maple
                std::fs::rename(temp_file, bin_path)?;

                println!("Latest version {latest_tag} download completed");
            } else {
                match asset_download_url(&latest_tag) {
                    Some(url) => {
                        println!("New maple release {latest_tag} is available, please download it from {url} or rerun with --download flag.");
                    }
                    None => {
                        println!("New maple release {latest_tag} is available, but no prebuilt binary provided for your platform");
                    }
                }
            }
        } else {
            println!("No newer release, current maple version: {latest_tag}");
        }

        Ok(())
    }
}

/// The prebuilt binary is put at bin/maple.
fn get_binary_path() -> std::io::Result<impl AsRef<std::path::Path>> {
    use std::io::{Error, ErrorKind};

    let exe_dir = std::env::current_exe()?;
    let bin_dir = exe_dir.parent().ok_or_else(|| {
        Error::new(
            ErrorKind::NotFound,
            "Parent directory of current executable not found",
        )
    })?;

    if !bin_dir.ends_with("bin") {
        return Err(Error::new(
            ErrorKind::Other,
            "Current executable is not from bin/***",
        ));
    }

    let bin_path = if cfg!(windows) {
        bin_dir.join("maple.exe")
    } else {
        bin_dir.join("maple")
    };

    Ok(bin_path)
}

/// Extracts the number of version from tag name, e.g., returns 13 out of the tag `v0.13`.
#[inline]
fn extract_remote_version_number(remote_tag: &str) -> u32 {
    remote_tag
        .split('.')
        .nth(1)
        .and_then(|s| s.parse().ok())
        .expect("Couldn't extract remote version")
}

/// local: "v0.13-4-g58738c0"
#[inline]
fn extract_local_version_number(local_tag: &str) -> u32 {
    let tag = local_tag.split('-').next().expect("Invalid local tag");
    extract_remote_version_number(tag)
}

#[cfg(unix)]
fn set_executable_permission<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path.as_ref())?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path.as_ref(), perms)?;
    Ok(())
}

/// Downloads the latest remote release binary to a temp file.
///
/// # Arguments
///
/// - `version`: "v0.13"
async fn download_prebuilt_binary(
    version: &str,
    no_progress_bar: bool,