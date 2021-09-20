#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod generator;
mod ping;
mod pixels;
mod startup;
mod utils;

use config::Config;
use std::{env, thread, time::Duration};
use tray::{Tray, TrayIcon};
use utils::NAME;

fn main() {
    let conf = Config::unmarshal();
    println!("{:?}", conf);

    if cfg!(not(debug_assertions)) && conf.autostart {
        startup::add_startup();
    }

    let mut tray = Tray::new().unwrap();
    tray.set_tooltip(NAME.to_owned() + "\nPinging: " + &conf.ip)
        .unwrap();

    let mut ico_path = env::temp_dir();
    ico_path.push(NAME);

    loop {
        let rtt = ping::get_rtt(&conf.ip);
        println!("RTT: {}", rtt);

        ico_path.push(rtt);
        if let Err(err) = tray.set_icon(ico_path.to_str().unwrap()) {
            eprintln!("{}", err);
            conf.generate_icons();
        };
        ico_path.pop();

        thread::sleep(Duration::from_secs(conf.delay));
    }
}
