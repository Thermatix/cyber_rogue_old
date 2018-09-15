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

use game::entity::Component;





fn main() {

//  intialize
    let settings = utility::config::Settings::new("config.toml");
    let manager: sys::EntityManager<Box<Component>> = sys::EntityManager::new();
    //


// main loop


// cleanup

}
