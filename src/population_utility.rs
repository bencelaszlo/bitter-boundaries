pub fn get_type_of_settlement(number_of_population: i32) -> String {
    if number_of_population > 10000000 {
        return String::from("megapolis");
    } else if number_of_population > 1000000 {
        return String::from("metropolis");
    } else if number_of_population > 500000 {
        return String::from("great city");
    } else if number_of_population > 100000 {
        return String::from("city");
    } else if number_of_population > 50000 {
        return String::from("big town");
    } else if number_of_population > 20000 {
        return String::from("town");
    } else if number_of_population > 10000 {
        return String::from("little town");
    } else if number_of_population > 5000 {
        return String::from("giant village");
    } else if number_of_population > 2000 {
        return String::from("large village");
    } else if number_of_population > 1000 {
        return String::from("village");
    } else if number_of_population > 500 {
        return String::from("small village");
    } else if number_of_population > 100 {
        return String::from("little village");
    } else {
        return String::from("hamlet");
    }
}

pub fn get_level_of_settlement(number_of_population: i32) -> i32 {
    if number_of_population > 10000000 {
        return 12;
    } else if number_of_population > 1000000 {
        return 11;
    } else if number_of_population > 500000 {
        return 10;
    } else if number_of_population > 100000 {
        return 9;
    } else if number_of_population > 50000 {
        return 8;
    } else if number_of_population > 20000 {
        return 7;
    } else if number_of_population > 10000 {
        return 6;
    } else if number_of_population > 5000 {
        return 5;
    } else if number_of_population > 2000 {
        return 4;
    } else if number_of_population > 1000 {
        return 3;
    } else if number_of_population > 500 {
        return 2;
    } else if number_of_population > 100 {
        return 1;
    } else {
        return 0;
    }
}

pub fn get_total_population(
    player: i32,
    tile_population: &Vec<Vec<i32>>,
    tile_owned_by: &Vec<Vec<i32>>,
    game_area_width: usize,
    game_area_height: usize,
) -> i32 {
    let mut total_population: i32 = 0;
    for i in 0..game_area_width {
        for j in 0..game_area_height {
            if tile_owned_by[i][j] == player {
                total_population += tile_population[i][j];
            }
        }
    }
    return total_population;
}

pub fn get_cash(total_population: i32) -> f64 {
    return 4.0 * (get_level_of_settlement(total_population) + 1) as f64;
}
