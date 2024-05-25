use crate::{BeansError, BUTLER_BINARY, BUTLER_LIB_1, BUTLER_LIB_2, helper};

/// try and write aria2c and butler if it doesn't exist
/// paths that are used will be fetched from binary_locations()
pub fn try_write_deps()
{
    safe_write_file(get_butler_location().as_str(), &**BUTLER_BINARY);
    safe_write_file(get_butler_1_location().as_str(), &**BUTLER_LIB_1);
    safe_write_file(get_butler_2_location().as_str(), &**BUTLER_LIB_2);
}
fn safe_write_file(location: &str, data: &[u8]) {
    if !helper::file_exists(location.to_string())
    {
        if let Err(e) = std::fs::write(&location, data) {
            eprintln!("[try_write_deps] failed to extract {}", location);
            if helper::do_debug() {
                eprintln!("[depends::try_write_deps] {:#?}", e);
            }
        }
        else
        {
            println!("[try_write_deps] extracted {}", location);
        }
    }
}

/// will not do anything since this only runs on windows
#[cfg(not(target_os = "windows"))]
pub async fn try_install_vcredist() -> Result<(), BeansError>
{
    // ignored since we aren't windows :3
    Ok(())
}
/// try to download and install vcredist from microsoft via aria2c
/// TODO use request instead of aria2c for downloading this.
#[cfg(target_os = "windows")]
pub async fn try_install_vcredist() -> Result<(), BeansError>
{
    let mut out_loc = std::env::temp_dir().to_str().unwrap_or("").to_string();
    if out_loc.ends_with("\\") == false {
        out_loc.push_str("\\");
    }
    out_loc.push_str("vc_redist.exe");
    helper::download_with_progress(
        String::from("https://aka.ms/vs/17/release/vc_redist.x86.exe"),
        out_loc.clone()).await?;

    if std::path::Path::new(&out_loc).exists() == false {
        return  Err(BeansError::FileNotFound {
            location: out_loc.clone()
        });
    }

    std::process::Command::new(&out_loc)
        .args(["/install","/passive","/norestart"])
        .spawn()
        .expect("Failed to install vsredist!")
        .wait()?;
    
    std::fs::remove_file(&out_loc)?;
    
    Ok(())
}

pub fn butler_exists() -> bool {
    helper::file_exists(get_butler_location())
    && helper::file_exists(get_butler_1_location())
    && helper::file_exists(get_butler_2_location())
}

pub fn get_butler_location() -> String
{
    let mut path = get_tmp_dir();
    path.push_str(BUTLER_LOCATION);
    path
}
pub fn get_butler_1_location() -> String {
    let mut path = get_tmp_dir();
    path.push_str(BUTLER_1);
    path
}
pub fn get_butler_2_location() -> String {
    let mut path = get_tmp_dir();
    path.push_str(BUTLER_2);
    path
}
fn get_tmp_dir() -> String {
    let mut path = std::env::temp_dir().to_str().unwrap_or("").to_string();
    if path.ends_with("/") == false && path.ends_with("\\") == false {
        #[cfg(target_os = "windows")]
        path.push_str("\\");
        #[cfg(not(target_os = "windows"))]
        path.push_str("/");
    }
    path
}

#[cfg(target_os = "windows")]
const BUTLER_LOCATION: &str = "butler.exe";
#[cfg(not(target_os = "windows"))]
const BUTLER_LOCATION: &str = "butler";

#[cfg(target_os = "windows")]
const BUTLER_1: &str = "7z.dll";
#[cfg(not(target_os = "windows"))]
const BUTLER_1: &str = "7z.so";
#[cfg(target_os = "windows")]
const BUTLER_2: &str = "c7zip.dll";
#[cfg(not(target_os = "windows"))]
const BUTLER_2: &str = "libc7zip.so";