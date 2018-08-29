use rand;

pub use tcod::console::*;
pub use tcod::colors;


mod entities;
mod stats;
mod manager;


pub mod ai;

pub use self::manager::Manager;
pub use self::entities::*;
pub use self::ai::Ai;
pub use self::stats::Stats;

pub mod emplacement;
// pub mod components;
pub mod contextual {
    pub const Block: bool = true;
    pub const NoBlock: bool  = false;
}


