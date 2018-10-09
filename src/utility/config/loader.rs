use config_rs::{ConfigError, Config, File};

#[derive(Debug, Deserialize)]
 pub struct Engine {
     pub screen_width: i32,
     pub screen_height: i32,
     pub fps_limit: i32,
}

#[derive(Debug, Deserialize)]
 pub struct Lighting {
     pub fov_light_walls: bool,
     pub torch_radius: i32,
}

#[derive(Debug, Deserialize)]
pub  struct Rooms {
     pub room_min_size: i32,
     pub room_max_size: i32,
     pub room_max_no: i32,
}

#[derive(Debug, Deserialize)]
 pub struct Monsters {
     pub max_monsters: i32,
}

#[derive(Debug, Deserialize)]
pub struct Entities {
    pub templates: String,
    pub lists: String,
    pub feature_packs: String,
}

#[derive(Debug, Deserialize)]
pub struct Directories {
    pub data: String,
    pub entities: Entities
}

#[derive(Debug, Deserialize)]
 pub struct Settings {
     pub engine: Engine,
     pub lighting: Lighting,
     pub rooms: Rooms,
     pub monsters: Monsters,
     pub dirs: Directories,
}

impl Settings {
    pub fn new(file: &str) -> Result<Self, ConfigError> {
        let mut settings = Config::new();
        settings.merge(File::with_name(file)).unwrap();
        settings.try_into()
    }
}
