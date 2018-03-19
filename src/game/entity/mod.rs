use rand;

pub use tcod::console::*;
pub use tcod::colors;


mod object;
pub use self::object::Object;
pub use self::object::Kind;

pub mod emplacement;
