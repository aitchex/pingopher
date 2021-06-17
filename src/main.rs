#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod generator;
mod ping;
mod pixels;
mod utils;

use config::Config;
use generator::Icon;
use std::{env, thread, time::Duration};
use tray::{Tray, TrayIcon};
use utils::NAME;

fn main() {
    let conf = Config::unmarshal();
    println!("{:?}", conf);

    let mut tray = Tray::new().unwrap();
    tray.set_tooltip(NAME).unwrap();

    let mut ico_path = env::temp_dir();
    ico_path.push(NAME);

    loop {
        let rtt = ping::get_rtt(&conf.ip);
        println!("RTT: {}", rtt);

        ico_path.push(rtt + ".ico");
        if let Err(err) = tray.set_icon(ico_path.to_str().unwrap()) {
            eprintln!("{}", err);
            Icon::generate(conf.color);
        };
        ico_path.pop();

        thread::sleep(Duration::from_secs(conf.delay));
    }
}
