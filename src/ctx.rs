use crate::{depends, helper, version};
use crate::helper::{find_sourcemod_path, InstallType};
use crate::version::{RemotePatch, RemoteVersion, RemoteVersionResponse};
use crate::wizard::BeansError;

#[derive(Debug, Clone)]
pub struct RunnerContext
{
    pub sourcemod_path: String,
    pub remote_version_list: RemoteVersionResponse,
    pub current_version: Option<usize>
}
impl RunnerContext
{
    pub async fn create_auto() -> Result<Self, BeansError>
    {
        depends::try_write_deps();
        depends::try_install_vcredist();
        let sourcemod_path = match find_sourcemod_path() {
            Some(v) => v,
            None => {
                return Err(BeansError::SourceModLocationNotFound);
            }
        };
        let version_list = crate::version::get_version_list().await;

        if helper::install_state() == InstallType::OtherSource {
            version::update_version_file();
        }

        return Ok(Self
        {
            sourcemod_path,
            remote_version_list: version_list,
            current_version: crate::version::get_current_version()
        });
    }

    /// Get the latest item in `remote_version_list`
    pub fn latest_remote_version(&mut self) -> (usize, RemoteVersion)
    {
        let mut highest = usize::MIN;
        for (key, _) in self.remote_version_list.clone().versions.into_iter() {
            if key > highest {
                highest = key;
            }
        }
        let x = self.remote_version_list.versions.get(&highest).unwrap();
        (highest, x.clone())
    }

    /// When self.current_version is some, iterate through patches and fetch the patch that is available
    /// to bring the current version in-line with the latest version.
    pub fn has_patch_available(&mut self) -> Option<RemotePatch>
    {
        let current_version = self.current_version.clone();
        let (remote_version, _) = self.latest_remote_version();
        match current_version {
            Some(cv) => {
                for (_, patch) in self.remote_version_list.clone().patches.into_iter() {
                    if patch.file == format!("of-{}to{}.pwr", cv, remote_version) {
                        return Some(patch);
                    }
                }
                return None;
            },
            _ => None
        }
    }
}