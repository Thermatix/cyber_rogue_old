use std::collections::HashMap;
use config_rs::{ConfigError, Config, File};

#[derive(Debug, Deserialize)]
enum InitialValues {
    String,
    i32,
    bool,
}

#[derive(Debug, Deserialize)]
struct Template {
    component: String,
    initial_value: InitialValues,
    list: bool,
}

type FileName = String;
type Name = String;
type Templates = HashMap<FileName, HashMap<Name,Template>>;


#[derive(Debug, Deserialize)]
pub struct Manager {
    templates: Templates,
}

impl Manager {
    pub fn new(file: &str) -> Result<Self, ConfigError> {
        let mut manager = Config::new();
        manager.merge(File::with_name(file)).unwrap();
        manager.try_into()
    }
}
