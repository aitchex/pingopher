mod config;
mod ping;

use config::Config;
use std::{thread, time};

fn main() {
    let conf = Config::unmarshal();
    println!("{:?}", conf);

    loop {
        let rtt = ping::get_rtt(&conf.ip);
        println!("{}", rtt);
        thread::sleep(time::Duration::from_secs(conf.delay));
    }
}
