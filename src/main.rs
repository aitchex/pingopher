mod config;
mod generator;
mod ping;
mod pixels;

use config::Config;
use std::{thread, time::Duration};

fn main() {
    let conf = Config::unmarshal();
    println!("{:?}", conf);

    loop {
        let rtt = ping::get_rtt(&conf.ip);
        println!("{}", rtt);
        thread::sleep(Duration::from_secs(conf.delay));
    }
}
