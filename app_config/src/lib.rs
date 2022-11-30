use app_domain::config::Config;

pub fn parse_config(path: &str) -> Config {
    let f = std::fs::File::open(path).expect("Could not open file.");
    serde_yaml::from_reader(f).expect("Could not read values.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
