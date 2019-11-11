pub struct TileCoordinates {
    pub x: usize,
    pub y: usize,
}

pub fn closest_player_settlement(
    tile_owned_by: &Vec<Vec<i32>>,
    enemy_tile_x: i32,
    enemy_tile_y: i32,
    game_area_width: usize,
    game_area_height: usize,
) -> TileCoordinates {
    let mut closest_player_tile: TileCoordinates = TileCoordinates {
        x: game_area_width,
        y: game_area_height,
    };
    let mut distance: i32 = 99;
    for i in 0..game_area_width {
        for j in 0..game_area_height {
            if tile_owned_by[i][j] == 0 {
                let new_distance: i32 = (enemy_tile_x - i as i32 * enemy_tile_x - i as i32)
                    + (enemy_tile_y - j as i32 * enemy_tile_y - j as i32);
                if new_distance < distance {
                    closest_player_tile.x = i;
                    closest_player_tile.y = j;
                    distance = new_distance;
                }
            }
        }
    }
    return closest_player_tile;
}
