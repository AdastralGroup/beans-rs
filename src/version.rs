use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Write;
use crate::helper;
use crate::helper::{find_sourcemod_path, InstallType};
use crate::BeansError;

/// get the current version installed via the .adastral file in the sourcemod mod folder.
/// will parse the value of `version` as usize.
pub fn get_current_version() -> Option<usize>
{
    let install_state = helper::install_state();
    if install_state != InstallType::Adastral {
        return None;
    }
    match get_mod_location() {
        Some(smp_x) => {
            // TODO generate BeansError instead of using .expect
            let location = format!("{}.adastral", smp_x);
            let content = read_to_string(&location).expect(format!("Failed to open {}", location).as_str());
            let data: AdastralVersionFile = serde_json::from_str(&content).expect(format!("Failed to deserialize data at {}", location).as_str());
            let parsed = data.version.parse::<usize>().expect(format!("Failed to convert version to usize! ({})", data.version).as_str());
            Some(parsed)
        },
        None => None
    }
}
fn get_version_location() -> Option<String>
{
    match get_mod_location() {
        Some(v) => Some(format!("{}.adastral", v)),
        None => None
    }
}
/// get the full location of the sourcemod mod directory.
fn get_mod_location() -> Option<String>
{
    let mut smp_x = match find_sourcemod_path() {
        Ok(v) => v,
        Err(e) => {
            if helper::do_debug() {
                eprintln!("[version::get_mod_location] {} {:#?}", BeansError::SourceModLocationNotFound, e);
            }
            return None;
        }
    };
    if smp_x.ends_with("/") || smp_x.ends_with("\\") {
        smp_x.pop();
    }
    smp_x.push_str(crate::DATA_DIR);
    Some(smp_x)
}
/// migrate from old file (.revision) to new file (.adastral) in sourcemod mod directory.
pub fn update_version_file()
{
    let install_state = helper::install_state();
    if install_state == InstallType::Adastral {
        return;
    }
    // ignore :)
    else if install_state == InstallType::OtherSourceManual {
        return;
    }
    // ignore :)
    else if install_state == InstallType::NotInstalled {
        return;
    }

    let mut smp_x = match find_sourcemod_path() {
        Ok(v) => v,
        Err(e) => {
            if helper::do_debug() {
                eprintln!("[version::update_version_file] {} {:#?}", BeansError::SourceModLocationNotFound, e);
            }
            return;
        }
    };
    if smp_x.ends_with("/") || smp_x.ends_with("\\") {
        smp_x.pop();
    }

    let old_version_file_location = format!("{}{}.revision", smp_x, crate::DATA_DIR);
    let old_version_file_content = read_to_string(&old_version_file_location).expect(format!("Failed to open {}", old_version_file_location).as_str());
    let old_version_idx = match old_version_file_content.parse::<usize>() {
        Ok(v) => v,
        Err(e) => {
            panic!("Failed to parse old version number from {}\nIt was; {}\n\n{:#?}", old_version_file_location, old_version_file_content, e);
        }
    };

    let new_file_content = AdastralVersionFile
    {
        version: old_version_idx.to_string()
    };

    let new_version_file_location = format!("{}{}.adastral", smp_x, crate::DATA_DIR);
    let new_version_file_content = match serde_json::to_string(&new_file_content) {
        Ok(v) => v,
        Err(e) => {
            panic!("Failed to serialize! {:#?}", e);
        }
    };
    std::fs::write(new_version_file_location, new_version_file_content).expect("Failed to migrate old file to new file!");
    std::fs::remove_file(old_version_file_location).expect("Failed to delete old version file");
}

/// fetch the version list from `{crate::SOURCE_URL}versions.json`
pub async fn get_version_list() -> RemoteVersionResponse
{
    let response = match reqwest::get(format!("{}versions.json", crate::SOURCE_URL)).await {
        Ok(v) => v,
        Err(e) => {
            panic!("Failed to get versions from server!\n{:#?}", e);
        }
    };
    let response_text = response.text().await.expect("Failed to get version details content");
    let data = serde_json::from_str(&response_text).expect("Failed to deserialize version details from server");

    return data;
}

/// Version file that is used as `.adastral` in the sourcemod mod folder.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AdastralVersionFile {
    pub version: String
}
impl AdastralVersionFile {
    pub fn write(&self) -> Result<(), BeansError> {
        match get_version_location() {
            Some(vl) => {
                let ex = helper::file_exists(vl.clone());
                match std::fs::OpenOptions::new()
                    .create_new(ex)
                    .write(true)
                    .append(false)
                    .open(&vl) {
                    Ok(mut file) => {
                        match serde_json::to_string(self) {
                            Ok(ser) => {
                                match file.write_all(ser.as_bytes()) {
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(BeansError::FileWriteFailure(vl, e))
                                }
                            },
                            Err(e) => Err(e.into())
                        }
                    },
                    Err(e) => Err(BeansError::FileOpenFailure(vl, e))
                }
            },
            None => Err(BeansError::SourceModLocationNotFound)
        }
    }
}
/// Value of the `versions` property in `RemoteVersionResponse`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteVersion
{
    pub url: Option<String>,
    pub file: Option<String>,
    #[serde(rename = "presz")]
    pub pre_sz: Option<usize>,
    #[serde(rename = "postsz")]
    pub post_sz: Option<usize>,
    #[serde(rename = "signature")]
    pub signature_url: Option<String>,
    #[serde(rename = "heal")]
    pub heal_url: Option<String>
}
/// `versions.json` response content from remote server.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteVersionResponse
{
    pub versions: HashMap<usize, RemoteVersion>,
    pub patches: HashMap<usize, RemotePatch>
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemotePatch
{
    pub url: String,
    pub file: String,
    /// Amount of file space required for temporary file. Assumed to be measured in bytes.
    pub tempreq: usize
}