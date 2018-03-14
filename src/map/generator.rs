mod Dungeon {
  mod Basic {

    pub fn create(map: &mut Map) -> (Map, (i32, i32)) {
      // fill map.data with "blocked" tiles

      let mut starting_position = (0, 0);

      for _ in 0..MAX_map.rooms {
        // random width and height
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        // random position without going out of the boundaries of the map.data
        let x = rand::thread_rng().gen_range(0, map.width - w);
        let y = rand::thread_rng().gen_range(0, map.height - h);

        let new_room = Rect::new(x, y, w, h);

        // run through the other map.rooms and see if they intersect with this one
        let failed = map.rooms.iter().any(|other_room| new_room.intersects_with(other_room));

        if !failed {
          // this means there are no intersections, so this room is valid

          // "paint" it to the map.data's tiles
          create_room(new_room, &mut map.data);

          // center coordinates of the new room, will be useful later
          let (new_x, new_y) = new_room.center();

          if map.rooms.is_empty() {
            // this is the first room, where the player starts at
            starting_position = (new_x, new_y);
          } else {
            // all map.rooms after the first:
            // connect it to the previous room with a tunnel

            // center coordinates of the previous room
            let (prev_x, prev_y) = map.rooms[rooms.len() - 1].center();

            // toss a coin (random bool value -- either true or false)
            if rand::random() {
              // first move horizontally, then vertically
              create_h_tunnel(prev_x, new_x, prev_y, &mut map.data);
              create_v_tunnel(prev_y, new_y, new_x, &mut map.data);
            } else {
              // first move vertically, then horizontally
              create_v_tunnel(prev_y, new_y, prev_x, &mut map.data);
              create_h_tunnel(prev_x, new_x, new_y, &mut map.data);
            }
          }

          // finally, append the new room to the list
          map.rooms.push(new_room);
        }
      }

      (map.data, starting_position)

    }


  }
}
