mod config;

use config::Config;

fn main() {
    let conf = Config::unmarshal();
    println!("{:?}", conf);
}
