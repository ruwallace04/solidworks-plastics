//! Windows registry read helpers for Solidworks Plastics.

pub const APP_KEY: &str = "Software\\Solidworks\\Solidworks Plastics";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Solidworks Plastics";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Solidworks Plastics") },
        RegValue { name: "Version",     value: String::from("v3.3.29") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Solidworks Plastics") },
        RegValue { name: "Publisher",       value: String::from("Solidworks") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Solidworks Plastics\\uninstall.exe\"") },
    ]
}
