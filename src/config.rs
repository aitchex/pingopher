use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::File,
    io::{BufReader, ErrorKind, Write},
    path::PathBuf,
};

#[derive(Debug, Serialize, Deserialize)]
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

// TODO: Return Result from associated functions
impl Config {
    fn get_path() -> PathBuf {
        let mut path = env::current_exe().expect("Getting current exe path failed");
        path.pop();
        path.push("config.json");

        path
    }

    fn create() {
        let json = serde_json::to_string_pretty(&Config::default()).unwrap();

        let mut file = match File::create(Self::get_path()) {
            Ok(file) => file,
            Err(err) => panic!("Creating config.json failed: {}", err),
        };

        file.write_all(json.as_bytes())
            .expect("Writing config.json failed");
    }

    pub fn unmarshal() -> Config {
        let file = match File::open(Self::get_path()) {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    Self::create();
                    return Config::default();
                }
                other_error => {
                    panic!("Opening config.json failed: {:?}", other_error)
                }
            },
        };

        let reader = BufReader::new(file);
        let conf = serde_json::from_reader(reader).unwrap();

        conf
    }
}
