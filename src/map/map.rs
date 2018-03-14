
type Layer = Vec<Vec<Tile>>;

pub struct Map {
  height: i32,
  width: i32,
  data: Layer,
  rooms: Vec<Rect>,
}

impl Map {
  pub fn new(width: i32, height: i32) -> Self {
    Map {height: height, width: width, data:vec![vec![Tile::wall(); height as usize]; width as usize], rooms: vec![Rect]}
  }
  pub fn generate_with(&self, creator: module {
    creator::generate(&self)
  }
}

