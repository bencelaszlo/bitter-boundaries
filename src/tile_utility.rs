use std::f32;

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
    player_id: i32,
) -> TileCoordinates {
    let mut closest_player_tile: TileCoordinates = TileCoordinates {
        x: game_area_width,
        y: game_area_height,
    };
    let mut x_start: usize = enemy_tile_x as usize;
    let mut x_end: usize = enemy_tile_x as usize;
    let mut y_start: usize = enemy_tile_y as usize;
    let mut y_end: usize = enemy_tile_y as usize;

    if enemy_tile_x > 0 {
        x_start -= 1;
    }

    if enemy_tile_x < game_area_width as i32 - 1 {
        x_end += 1;
    }

    if enemy_tile_y > 0 {
        y_start -= 1;
    }

    if enemy_tile_y < game_area_height as i32 - 1 {
        y_end += 1;
    }

    let mut x_distance: f32 = game_area_width as f32;
    let mut y_distance: f32 = game_area_height as f32;

    for i in x_start..x_end + 1 {
        for j in y_start..y_end + 1 {
            if i != enemy_tile_x as usize
                && j != enemy_tile_y as usize
                && tile_owned_by[i][j] == player_id
            {
                let new_x_distance: f32 = (enemy_tile_x as f32 - i as f32).abs();
                let new_y_distance: f32 = (enemy_tile_y as f32 - i as f32).abs();
                if new_x_distance <= x_distance && new_y_distance <= y_distance {
                    closest_player_tile.x = i;
                    closest_player_tile.y = j;
                    x_distance = new_x_distance;
                    y_distance = new_y_distance;
                }
            }
        }
    }

    return closest_player_tile;
}

pub fn has_adjacent_friendly_tile(
    tile_owned_by: &Vec<Vec<i32>>,
    enemy_tile_x: i32,
    enemy_tile_y: i32,
    game_area_width: usize,
    game_area_height: usize,
    player_id: i32,
) -> bool {
    let mut x_start: usize = enemy_tile_x as usize;
    let mut x_end: usize = enemy_tile_x as usize;
    let mut y_start: usize = enemy_tile_y as usize;
    let mut y_end: usize = enemy_tile_y as usize;

    if enemy_tile_x > 0 {
        x_start -= 1;
    }

    if enemy_tile_x < game_area_width as i32 - 2 {
        x_end += 1;
    }

    if enemy_tile_y > 0 {
        y_start -= 1;
    }

    if enemy_tile_y < game_area_height as i32 - 2 {
        y_end += 1;
    }

    let mut result = false;

    for i in x_start..x_end + 1 {
        for j in y_start..y_end + 1 {
            if i != enemy_tile_x as usize
                && j != enemy_tile_y as usize
                && tile_owned_by[i][j] == player_id
            {
                result = true;
            }
        }
    }

    return result;
}

pub fn is_player_wins(
    tile_owned_by: &Vec<Vec<i32>>,
    game_area_width: usize,
    game_area_height: usize,
    player: i32,
) -> bool {
    let mut player_wins = true;
    for i in 0..game_area_width {
        for j in 0..game_area_height {
            if tile_owned_by[i][j] != player {
                player_wins = false;
            }
        }
    }
    return player_wins;
}
