//config crates
extern crate config as config_rs;
extern crate serde;

// utlity crates
extern crate nanoid;
#[macro_use]
extern crate serde_derive;

extern crate typemap;

mod utility;
mod sys;
mod game;


fn main() {

//  intialize
    let settings = utility::config::Settings::new("config.toml");
    let manager = sys::EntityManager::new();
    //


// main loop


// cleanup

}
