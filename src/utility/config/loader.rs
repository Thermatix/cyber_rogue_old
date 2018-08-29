use config_rs::{ConfigError, Config, File};
use super::structs;

impl structs::Settings {
    pub fn new(file: String) -> Result<Self, ConfigError> {
        let mut settings = Config::new();
        settings.merge(File::with_name(file.as_str())).unwrap();
        settings.try_into()
    }
}
