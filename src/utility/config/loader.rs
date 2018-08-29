use config_rs::{ConfigError, Config, File};
use super::structs;

impl structs::Settings {
    pub fn new(file: &str) -> Result<Self, ConfigError> {
        let mut settings = Config::new();
        settings.merge(File::with_name(file)).unwrap();
        settings.try_into()
    }
}
