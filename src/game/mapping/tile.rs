/// A tile of the map and its properties
#[derive(Clone, Copy, Debug)]
pub struct Tile {
  pub  blocked: bool,
  pub  block_sight: bool,
  pub  explored: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Self {blocked: false, block_sight: false, explored: false}
    }

    pub fn wall() -> Self {
        Self {blocked: true, block_sight: true, explored: false}
    }
}
