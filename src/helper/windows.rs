﻿use winreg::enums::HKEY_CURRENT_USER;
use std::backtrace::Backtrace;
use winreg::RegKey;
use crate::BeansError;
use crate::helper::generate_rand_str;

/// TODO use windows registry to get the SourceModInstallPath
/// HKEY_CURRENT_USER\Software\Value\Steam
/// Key: SourceModInstallPath
pub fn find_sourcemod_path() -> Result<String, BeansError>
{
    match RegKey::predef(HKEY_CURRENT_USER).open_subkey(String::from("Software\\Valve\\Steam")) {
        Ok(rkey) => {
            let x: std::io::Result<String> = rkey.get_value("SourceModInstallPath");
            match x {
                Ok(mut val) => {
                    if val.ends_with("\\") == false {
                        val.push_str("\\");
                    }
                    Ok(val)
                },
                Err(e) => {
                    return Err(BeansError::RegistryKeyFailure {
                        msg: "Failed to find HKCU\\Software\\Valve. Steam might not be installed".to_string(),
                        error: e,
                        backtrace: Backtrace::capture()
                    });
                }
            }
        },
        Err(e) => {
            return Err(BeansError::RegistryKeyFailure {
                msg: "Failed to find HKCU\\Software\\Valve. Steam might not be installed".to_string(),
                error: e,
                backtrace: Backtrace::capture()
            });
        }
    }
}