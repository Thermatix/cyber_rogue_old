pub mod mapping;
pub mod entity;

pub use game::mapping::*;
pub use game::entity::*;

pub mod types {
  use game::mapping::Tile;
  pub type Grid = Vec<Vec<Tile>>;
  pub type Location = (i32,i32);
  pub type Point = (i32,i32);
}

