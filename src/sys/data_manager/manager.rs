use std::fs;
use std::ops::Range;
use std::collections::HashMap;
use std::ops::{Index};

use crate::utility::{config::Settings, config::loader::Directories, list};

// use serde::Deserializer;
use crate::config_rs::{ConfigError, Config, File};

use game::entity::*;

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(
//     rename_all = "lowercase",
//     untagged
// )]
// pub enum InitialValue {
//     Char(char),
//     String(String),
//     Int(i32),
//     Float(f32),
//     Bool(bool),
//     Range(Range<i32>),
//     Point((i32,i32))
// }

// impl InitialValue {
//
//     pub fn unwrap_char(&self) -> &char {
//         match &self {
//             InitialValue::Char(val) => val,
//             _ => panic!("Stored value does not match unwrap type")
//         }
//     }
//
//     pub fn unwrap_string(&self) -> &String {
//         match &self {
//             InitialValue::String(val) => val,
//             _ => panic!("Stored value does not match unwrap type")
//         }
//     }
//
//     pub fn unwrap_int(&self) -> &i32 {
//         match &self {
//             InitialValue::Int(val) => val,
//             _ => panic!("Stored value does not match unwrap type")
//         }
//     }
//
//     pub fn unwrap_float(&self) -> &f32 {
//         match &self {
//             InitialValue::Float(val) => val,
//             _ => panic!("Stored value does not match unwrap type")
//         }
//     }
//
//     pub fn unwrap_bool(&self) -> &bool {
//         match &self {
//             InitialValue::Bool(val) => val,
//             _ => panic!("Stored value does not match unwrap type")
//         }
//     }
//
//     pub fn unwrap_range(&self) -> &Range<i32> {
//         match &self {
//             InitialValue::Range(val) => val,
//             _ => panic!("Stored value does not match unwrap type")
//         }
//     }
//
//     pub fn unwrap_point(&self) -> &(i32, i32) {
//         match &self {
//             InitialValue::Point(val) => val,
//             _ => panic!("Stored value does not match unwrap type")
//         }
//     }
// }

// #[derive(Debug, Deserialize)]
// pub struct Component {
//     #[serde(rename="component")]
//     name: String,
//     #[serde(default)]
//     initial_value: Option<InitialValue>,
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(
    rename_all = "lowercase",
    tag="component"
)]
pub enum Feature {
    Char        {
        value: <Char as Component>::ValueType
    },
    Kind        {
        value: <Kind as Component>::ValueType
    },
    Location    {
        value: <Location as Component>::ValueType
    },
    // TODO: figure out why this is not working?
    // Name        { value: <Name as Component>::ValueType },
    Name        {
        value: String
    },
    Position    {
        value: <Position as Component>::ValueType
    },
}

type Features = Vec<Feature>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Prototype {
    pub feature_packs: Vec<String>,
    pub features: Features,
}


type Name = String;
type Type = String;


#[derive(Debug, Serialize, Deserialize)]
pub struct Prototypes {
    #[serde(flatten)]
    data: HashMap<Name,Prototype>
}

impl Prototypes {
    /// For a given file load and parse the data into a list of prototypes
    pub fn new(file: &str) -> Result<Self, ConfigError> {
        let mut prototypes = Config::new();
        prototypes.merge(File::with_name(file)).unwrap();
        prototypes.try_into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeaturePacks {
    #[serde(flatten)]
    data: HashMap<Name, Features>
}

impl FeaturePacks {
    pub fn new(dirs: &Directories) -> Result<Self, ConfigError> {
        let mut feature_packs = Config::new();
        feature_packs
            .merge(File::with_name(&format!("{}{}", &dirs.data, &dirs.entities.feature_packs)))
            .unwrap();
        feature_packs.try_into()
    }
}


//                            Category     List Name   list of strings
type ListCategories = HashMap<String,HashMap<String ,Vec<String>>>;

#[derive(Debug, Deserialize)]
pub struct Lists {
    categories: ListCategories,
}

fn load_lists( list_dir: &str) -> ListCategories {
    let mut categories = ListCategories::new();
    'categories: for raw_dir in fs::read_dir(list_dir).unwrap() {
        let dir = raw_dir.unwrap().path();
        let mut category = HashMap::new();
        'lists: for raw_file in fs::read_dir(&dir).unwrap() {
            let file = raw_file.unwrap().path();
            category.insert( file.clone().file_stem().unwrap().to_str().unwrap().to_owned(), list::from_file(file).unwrap());
        }

        categories.insert(dir.file_stem().unwrap().to_str().unwrap().to_owned(), category);

    }
    categories
}

#[derive(Debug)]
pub struct Manager {
    pub prototypes: HashMap<Type,Prototypes>,
    pub feature_packs:FeaturePacks,
    pub lists: ListCategories,
}

impl Manager {
    /// For a given directory, itterate over files and load in the prototypes
    /// and return a Prototype maanger
    pub fn new(config: &Settings) -> Self {
        let mut manager = Self {
            prototypes: HashMap::new(),
            feature_packs: match FeaturePacks::new(&config.dirs) {
                Ok(fp) => fp,
                Err(message) => {
                    display_prototype_error(&format!("{},{}", &config.dirs.data, &config.dirs.entities.feature_packs), message.to_string());
                    panic!();
                }
            },
            lists: load_lists(&format!("{}{}", &config.dirs.data, &config.dirs.entities.lists))
        };

        'prototypes: for raw_path in fs::read_dir(&format!("{}{}", &config.dirs.data, &config.dirs.entities.prototypes)).unwrap() {
            let path = raw_path.unwrap().path();
            let file_path = path.clone();
            let type_name = path.clone();
            let prototypes =
                match Prototypes::new(type_name.to_str().unwrap()) {
                    Ok(prototypes) => prototypes,
                    Err(message) => {
                        display_prototype_error(file_path.clone().to_str().unwrap(), message.to_string());
                        continue 'prototypes
                    },
                };
            manager.prototypes.insert(file_path.file_stem().unwrap().to_str().unwrap().to_owned(), prototypes);
        };
        manager
    }
}

impl Index<Type> for Manager {
    type Output = Prototypes;

    /// Returns a reference to prototype type
    fn index(&self, t_type: Type) -> &Self::Output {
        &self.prototypes[&t_type]
    }

}

impl Index<Name> for Prototypes {
    type Output = Prototype;

    /// Returns a reference to prototype type
    fn index(&self, name: Name) -> &Self::Output {
        &self[name.to_string()]
    }

}

fn display_prototype_error(file: &str, message: String) {
    println!("Prototype loading failed for file {:?} because {}", file, message)
}
