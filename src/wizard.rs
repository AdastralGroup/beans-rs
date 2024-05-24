use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fs::read_to_string;
use crate::helper;
use crate::helper::{find_sourcemod_path, InstallType};
use crate::version::RemoteVersionResponse;

pub struct WizardContext
{
    pub sourcemod_path: String,
    pub remote_version_list: RemoteVersionResponse,
    pub current_version: Option<usize>
}
impl WizardContext
{
    /// run the wizard!
    pub async fn run()
    {
        helper::try_write_deps();
        helper::try_install_vcredist();
        let sourcemod_path = get_path();
        let version_list = crate::version::get_version_list().await;

        if helper::install_state() == InstallType::OtherSource {
            crate::version::update_version_file();
        }

        let mut i = Self
        {
            sourcemod_path,
            remote_version_list: version_list,
            current_version: crate::version::get_current_version()
        };
        i.menu().await;
    }

    /// Show the menu
    /// When an invalid option is selected, this will be re-called.
    pub async fn menu(&mut self)
    {
        println!();
        println!("1 - Install or reinstall the game");
        println!("2 - Check for and apply and available updates");
        println!("3 - Verify and repair game files");
        println!();
        println!("q - Quit");
        let user_input = helper::get_input("-- Enter option below --");
        match user_input.to_lowercase().as_str() {
            "1" => self.task_install().await,
            "2" => self.task_update().await,
            "3" => self.task_verify().await,
            "q" => std::process::exit(0),
            _ => {
                println!("Unknown option \"{}\"", user_input);
                self.menu().await;
            }
        }
    }
    /// Install the target game.
    pub async fn task_install(&mut self)
    {
        todo!()
    }
    /// Check for any updates, and if there are any, we install them.
    pub async fn task_update(&mut self)
    {
        todo!()
    }
    /// Verify the current data for the target sourcemod.
    pub async fn task_verify(&mut self)
    {
        todo!()
    }
}



fn get_path() -> String
{
    let current_path = helper::find_sourcemod_path();
    if let Some(x) = current_path {
        println!("Found sourcemods directory!\n{}", x);
        return x;
    }
    todo!();
}

