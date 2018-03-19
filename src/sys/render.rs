use sys::console::*;
use sys::FovAlgorithm;

use game::mapping::Map;
use game::mapping::FovMap;

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;




pub fn fov_check(map: &Map) -> FovMap {

  let mut fov_map = FovMap::new(::MAP_WIDTH, ::MAP_HEIGHT);

  for x in 0..map.width {
    for y in 0..map.height {
      fov_map.set(x,y,
                  !map[x as usize][y as usize].block_sight,
                  !map[x as usize][y as usize].blocked);
    }
  };
  fov_map
}


pub fn all(root: &mut Root, con: &mut Offscreen, map: &mut Map, fov_map: &mut FovMap) {

  // go through all tiles, and set their background color
  fov_map.compute_fov(map.objects[0].x, map.objects[0].y, ::CONFIG.torch_radius, ::CONFIG.fov_light_walls,FOV_ALGO);
  map.render(con, &fov_map);

  // draw all objects in the list
  for object in &map.objects {
    if fov_map.is_in_fov(object.x, object.y) {
      object.draw(con);
    }
  }

  // blit the contents of "con" to the root console
  blit(con, (0, 0), (map.width, map.height), root, (0, 0), 1.0, 1.0);

}
