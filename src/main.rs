mod config;
mod generator;
mod ping;
mod pixels;

use config::Config;
use generator::Icon;
use std::{env, thread, time::Duration};
use tray::{Tray, TrayIcon};

fn main() {
    let conf = Config::unmarshal();
    println!("{:?}", conf);

    let mut tray = Tray::new();
    tray.set_tooltip("Pingopher");

    let mut ico_path = env::temp_dir();
    ico_path.push("Pingopher");

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
