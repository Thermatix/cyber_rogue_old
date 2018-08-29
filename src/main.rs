//config crates
extern crate config as config_rs;
extern crate serde;

// utlity crates
extern crate nanoid;
#[macro_use]
extern crate serde_derive;

mod utility;
mod sys;
mod game;





fn main() {

//  intialize
    let settings = utility::config::Settings::new("config.toml".to_string());
    let manager = sys::entity_manager::Manager::new();
    //


// main loop


// cleanup

}
