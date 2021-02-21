use super::Component;

#[derive(Debug)]
struct Position {
  x: i32,
  y: i32
}

impl Component for Position {
  type Storage = VecStorage<Self>;
}



#[derive(Debug)]
struct Vevolicty {
  x: i32,
  y: i32,
}

impl Component for Vevolicty {
  type Storage = VecStorage<Self>;
}

#[derive(Debug)]

struct Ai;

impl Component for Ai {
  type Storage = NullStorage<Self>;
}
