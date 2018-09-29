use std::fs;
use std::ops::Range;
use std::collections::HashMap;

use serde::Deserializer;
use config_rs::{ConfigError, Config, File};

#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize)]
pub enum InitialValue {
    char(char),
    string(String),
    int(i32),
    float(f32),
    bool(bool),
    range(Range<i32>)
}

#[derive(Debug, Deserialize)]
pub struct Component {
    component: String,
    initial_value: InitialValue,
}


#[derive(Debug, Deserialize)]
pub struct Template {
    feature_packs: Vec<String>,
    components: Vec<Component>,
}


type Name = String;
type Type = String;


#[derive(Debug, Deserialize)]
pub struct Templates {
    #[serde(flatten)]
    templates: HashMap<Name,Template>,
}

impl Templates {
    pub fn new(file: &str) -> Result<Self, ConfigError> {
        let mut  templates = Config::new();
        templates.merge(File::with_name(file)).unwrap();
        templates.try_into()
    }
}

#[derive(Debug)]
pub struct Manager {
    pub templates: HashMap<Type,Templates>,
}

impl Manager {
    pub fn new(dir: &str) -> Self {
        let mut manager = Self { templates: HashMap::new() };
        'paths: for raw_path in fs::read_dir(dir).unwrap() {
            let path = raw_path.unwrap().path();
            let file_path = path.clone();
            let type_name = path.clone();
            let templates =
                match Templates::new(type_name.to_str().unwrap()) {
                    Ok(templates) => templates,
                    Err(message) => {
                        println!("TemplateManager: file({:?}), {}", &file_path, message);
                        continue 'paths
                    },
                };
            manager.templates.insert(file_path.file_stem().unwrap().to_str().unwrap().to_owned(), templates);
        };
        manager
    }
}
