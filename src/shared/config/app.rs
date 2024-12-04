use std::fs::File;
use std::io::Read;
use toml::Table;

pub struct Config {
    pub world_url: String,
    pub battlefield_url: String,
}

pub fn read_config(file_path: String) -> Config {
    let mut opened =
        File::open(file_path.clone()).expect(format!("{} doesnt exists", file_path).as_str());
    let mut config_buffer = [0u8; 4096];
    let got = opened.read(&mut config_buffer).unwrap();
    let toml = String::from_utf8(config_buffer[..got].to_vec())
        .unwrap()
        .as_str()
        .parse::<Table>()
        .expect("Invalid config format.");
    Config {
        world_url: toml["world_url"].as_str().unwrap().to_string(),
        battlefield_url: toml["battlefield_url"].as_str().unwrap().to_string(),
    }
}

#[cfg(test)]
mod configurator {
    use crate::application::app::read_config;

    #[test]
    fn test_config() {
        let config = read_config("config_sample.toml".to_string());
        assert_eq!(config.world_url, "http://localhost:8002".to_string());
        assert_eq!(config.battlefield_url, "http://localhost:8000".to_string())
    }
}
