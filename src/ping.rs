use std::{os::windows::process::CommandExt, process::Command};

const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn get_rtt(ip: &String) -> String {
    let output = Command::new("cmd")
        .args(&["/C", "ping -n 1", ip])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .expect("Failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();

    let eq_index = match output.rfind("=") {
        Some(index) => index,
        None => return String::from("err"),
    };
    let ms_index = match output.rfind("ms") {
        Some(index) => index,
        None => return String::from("err"),
    };

    output[eq_index + 2..ms_index].to_string()
}
