use crate::{BeansError, butler, helper, RunnerContext};
use crate::version::RemoteVersion;

pub struct VerifyWorkflow {
    pub ctx: RunnerContext
}
impl VerifyWorkflow {
    pub async fn wizard(ctx: &mut RunnerContext) -> Result<(), BeansError>
    {
        let av = crate::appvar::parse();

        let current_version_id = match ctx.current_version {
            Some(v) => v,
            None => {
                println!("[VerifyWorkflow::wizard] Unable to update game since it is not installed!");
                return Ok(());
            }
        };

        let remote: RemoteVersion = ctx.current_remote_version()?;
        if remote.signature_url.is_none() {
            eprintln!("[VerifyWorkflow::wizard] Couldn't get signature URL for version {}", current_version_id);
        }
        if remote.heal_url.is_none() {
            eprintln!("[VerifyWorkflow::wizard] Couldn't get heal URL for version {}", current_version_id);
        }
        if remote.signature_url.is_none() || remote.heal_url.is_none() {
            eprintln!("[VerifyWorkflow::wizard] Unable to update, missing remote files!");
            return Ok(());
        }

        ctx.gameinfo_perms()?;
        let gameinfo_backup = ctx.read_gameinfo_file()?;
        let mod_dir_location = ctx.get_mod_location();
        butler::verify(
            format!("{}{}", &av.remote_info.base_url, remote.signature_url.unwrap()),
            mod_dir_location.clone(),
            format!("{}{}", &av.remote_info.base_url, remote.heal_url.unwrap()))?;
        ctx.gameinfo_perms()?;
        if let Some(gi) = gameinfo_backup {
            helper::restore_gameinfo(ctx, gi)?;
        }
        println!("[VerifyWorkflow::wizard] The verification process has completed, and any corruption has been repaired.");
        Ok(())
    }
}