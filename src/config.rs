use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub ip: String,
    pub delay: u64,
    pub color: [u8; 3],
    pub autostart: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ip: String::from("1.1.1.1"),
            delay: 1,
            color: [255, 255, 255],
            autostart: true,
        }
    }
}
