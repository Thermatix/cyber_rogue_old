
type Layer = Vec<Vec<Tile>>;

pub struct Map {
  height: i32,
  width: i32,
  data: Layer,
  rooms: Vec<Rect>,
}

impl Map {
  pub fn new(height: i32, width: i32) -> Self {
    Map {height: height, width: width, data:vec![vec![Tile::wall(); height as usize]; width as usize]}
  }
}

pub fn make_map() -> (Map, (i32, i32)) {
  // fill map with "blocked" tiles
  let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

  let mut rooms = vec![];

  let mut starting_position = (0, 0);

  for _ in 0..MAX_ROOMS {
    // random width and height
    let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
    let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
    // random position without going out of the boundaries of the map
    let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
    let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

    let new_room = Rect::new(x, y, w, h);

    // run through the other rooms and see if they intersect with this one
    let failed = rooms.iter().any(|other_room| new_room.intersects_with(other_room));

    if !failed {
      // this means there are no intersections, so this room is valid

      // "paint" it to the map's tiles
      create_room(new_room, &mut map);

      // center coordinates of the new room, will be useful later
      let (new_x, new_y) = new_room.center();

      if rooms.is_empty() {
        // this is the first room, where the player starts at
        starting_position = (new_x, new_y);
      } else {
        // all rooms after the first:
        // connect it to the previous room with a tunnel

        // center coordinates of the previous room
        let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

        // toss a coin (random bool value -- either true or false)
        if rand::random() {
          // first move horizontally, then vertically
          create_h_tunnel(prev_x, new_x, prev_y, &mut map);
          create_v_tunnel(prev_y, new_y, new_x, &mut map);
        } else {
          // first move vertically, then horizontally
          create_v_tunnel(prev_y, new_y, prev_x, &mut map);
          create_h_tunnel(prev_x, new_x, new_y, &mut map);
        }
      }

      // finally, append the new room to the list
      rooms.push(new_room);
    }
  }

  (map, starting_position)

}


