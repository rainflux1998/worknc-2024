//! Windows registry read helpers for Worknc 2024.

pub const APP_KEY: &str = "Software\\Worknc\\Worknc 2024";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Worknc 2024";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Worknc 2024") },
        RegValue { name: "Version",     value: String::from("v1.1.14") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Worknc 2024") },
        RegValue { name: "Publisher",       value: String::from("Worknc") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Worknc 2024\\uninstall.exe\"") },
    ]
}
