pub mod mapping;
pub mod entity;

pub use self::mapping::*;
// pub use self::entity::*;

pub mod types {
  pub type Grid<T> = Vec<Vec<T>>;
  pub type Location = (i32,i32);
  pub type Point = (i32,i32);
}

