use std::fs;
use std::ops::Range;
use std::collections::HashMap;
use std::ops::{Index};

// use serde::Deserializer;
use config_rs::{ConfigError, Config, File};

#[derive(Debug, Deserialize)]
#[serde(
    rename_all = "lowercase",
    untagged
)]
pub enum InitialValue {
    Char(char),
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    Range(Range<i32>)
}

#[derive(Debug, Deserialize)]
pub struct Component {
    component: String,
    initial_value: InitialValue,
}


#[derive(Debug, Deserialize)]
pub struct Template {
    pub feature_packs: Vec<String>,
    pub components: Vec<Component>,
}


type Name = String;
type Type = String;


#[derive(Debug, Deserialize)]
pub struct Templates {
    #[serde(flatten)]
    templates: HashMap<Name,Template>,
}

impl Templates {
    /// For a given file load and parse the data into a list of templates
    pub fn new(file: &str) -> Result<Self, ConfigError> {
        let mut  templates = Config::new();
        templates.merge(File::with_name(file)).unwrap();
        templates.try_into()
    }
}

#[derive(Debug)]
pub struct Manager {
    pub template_types: HashMap<Type,Templates>,
}

impl Manager {
    /// For a given directory, itterate over files and load in the templates
    /// and return a Template maanger
    pub fn new(dir: &str) -> Self {
        let mut manager = Self { template_types: HashMap::new() };
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
            manager.template_types.insert(file_path.file_stem().unwrap().to_str().unwrap().to_owned(), templates);
        };
        manager
    }
}

impl Index<Type> for Manager {
    type Output = Templates;

    /// Returns a reference to template type
    fn index(&self, t_type: Type) -> &Self::Output {
        &self.template_types[&t_type]
    }

}

impl Index<Name> for Templates {
    type Output = Template;

    /// Returns a reference to template type
    fn index(&self, name: Name) -> &Self::Output {
        &self.templates[&name]
    }

}
