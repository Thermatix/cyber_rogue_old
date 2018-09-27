use config_rs::{ConfigError, Config, File};

#[derive(Debug, Deserialize)]
 struct Engine {
     screen_width: i32,
     screen_height: i32,
     fps_limit: i32,
}

#[derive(Debug, Deserialize)]
 struct Lighting {
     fov_light_walls: bool,
     torch_radius: i32,
}

#[derive(Debug, Deserialize)]
 struct Rooms {
     room_min_size: i32,
     room_max_size: i32,
     room_max_no: i32,
}

#[derive(Debug, Deserialize)]
 struct Monsters {
     max_monsters: i32,
}

#[derive(Debug, Deserialize)]
 pub struct Settings {
     engine: Engine,
     lighting: Lighting,
     rooms: Rooms,
     monsters: Monsters,
}

impl Settings {
    pub fn new(file: &str) -> Result<Self, ConfigError> {
        let mut settings = Config::new();
        settings.merge(File::with_name(file)).unwrap();
        settings.try_into()
    }
}
