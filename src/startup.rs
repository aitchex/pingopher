use std::env;

use winreg::{enums::HKEY_CURRENT_USER, RegKey};

use crate::utils::NAME;

pub fn add_startup() {
    let hkey = RegKey::predef(HKEY_CURRENT_USER);

    let path = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
    let (key, _) = hkey.create_subkey(path).expect("Opening subkey failed");

    if let Err(_) = key.get_raw_value(NAME) {
        let exe = env::current_exe().expect("Getting current exe path failed");
        key.set_value(NAME, &exe.as_os_str())
            .expect("Setting startup registry value failed");
    };
}
