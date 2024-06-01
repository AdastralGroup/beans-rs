use crate::{DownloadFailureReason, helper, RunnerContext};
use crate::BeansError;
use crate::version::AdastralVersionFile;

#[derive(Debug, Clone)]
pub struct InstallWorkflow {
    pub context: RunnerContext
}
impl InstallWorkflow {
    pub async fn wizard(ctx: &mut RunnerContext) -> Result<(), BeansError>
    {
        let (latest_remote_id, latest_remote) = ctx.latest_remote_version();
        if let Some(_cv) = ctx.current_version {
            println!("[InstallWorkflow::wizard] re-installing! game files will not be touched until extraction");
        }

        let presz_loc = RunnerContext::download_package(latest_remote).await?;
        Self::install_from(presz_loc.clone(), ctx.sourcemod_path.clone(), Some(latest_remote_id)).await?;
        if helper::file_exists(presz_loc.clone()) {
            std::fs::remove_file(presz_loc)?;
        }
        Ok(())
    }

    /// Install the `.tar.zstd` file at `package_loc` to `out_dir`
    /// package_loc: Location to a file that is a `.tar.zstd` file.
    /// out_dir: should be `RunnerContext.sourcemod_path`
    /// version_id: Version that is from `package_loc`. When not specified, `.adastral` will not be written to.
    /// Note: This function doesn't check the extension when extracting.
    pub async fn install_from(package_loc: String, out_dir: String, version_id: Option<usize>) -> Result<(), BeansError>
    {
        if helper::file_exists(package_loc.clone()) == false {
            eprintln!("[InstallWorkflow::Wizard] Failed to find package! (location: {package_loc})");
            return Err(BeansError::DownloadFailure {
                reason: DownloadFailureReason::FileNotFound {
                    location: package_loc.clone()
                }
            });
        }

        println!("[InstallWorkflow::Wizard] Extracting to {out_dir}");
        RunnerContext::extract_package(package_loc, out_dir.clone())?;
        if let Some(lri) = version_id {
            let x = AdastralVersionFile {
                version: lri.to_string()
            }.write(Some(out_dir.clone()));
            if let Err(e) = x {
                println!("[InstallWorkflow::install_from] Failed to set version to {} in .adastral", lri);
                if helper::do_debug() {
                    eprintln!("{:#?}", e);
                }
            }
        } else {
            eprintln!("Not writing .adastral since the version wasn't provided");
        }
        println!("{}", INSTALL_FINISH_MSG);
        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
pub const INSTALL_FINISH_MSG: &str = include_str!("../text/install_complete_linux.txt");
#[cfg(target_os = "windows")]
pub const INSTALL_FINISH_MSG: &str = include_str!("../text/install_complete_windows.txt");