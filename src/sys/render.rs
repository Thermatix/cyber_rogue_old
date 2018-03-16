use sys::console::*;
use sys::FovAlgorithm;

use game::mapping::Map;
use game::mapping::FovMap;
use game::Object;

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;




pub fn fov_check(map: &Map) -> FovMap {

  let mut fov_map = FovMap::new(::MAP_WIDTH, ::MAP_HEIGHT);

  for x in 0..map.width {
    for y in 0..map.height {
      fov_map.set(x,y,
                  !map.data[x as usize][y as usize].block_sight,
                  !map.data[x as usize][y as usize].blocked);
    }
  };
  fov_map
}


pub fn all(root: &mut Root, con: &mut Offscreen, objects: &[Object], map: &mut Map, fov_map: &mut FovMap) {

  // go through all tiles, and set their background color
  fov_map.compute_fov(objects[0].x, objects[0].y, ::CONFIG.torch_radius, ::CONFIG.fov_light_walls,FOV_ALGO);
  map.render(con, &fov_map);

  // draw all objects in the list
  for object in objects {
    if fov_map.is_in_fov(object.x, object.y) {
      object.draw(con);
    }
  }

  // blit the contents of "con" to the root console
  blit(con, (0, 0), (map.width, map.height), root, (0, 0), 1.0, 1.0);

}
