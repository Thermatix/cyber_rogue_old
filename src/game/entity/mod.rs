use rand;

pub use tcod::console::*;
pub use tcod::colors;


pub mod object;
pub mod ai;
pub mod stats;


pub use self::object::Object;
pub use self::object::Kind;
pub use self::ai::Ai;
pub use self::stats::Stats;

pub mod emplacement;
