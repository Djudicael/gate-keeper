use app_config::{self, parse_config};
fn main() {
    let config = parse_config("config.yml");
    println!("{:?}", config);
}
