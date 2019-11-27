mod population_utility;
mod tile_utility;

extern crate image;
extern crate quicksilver;
extern crate rand;

use quicksilver::{
    combinators::result,
    geom::{Rectangle, Shape, Vector},
    graphics::{Background::Img, Color, Font, FontStyle, Image, View},
    input::{ButtonState, Key, MouseButton},
    lifecycle::{run, Asset, Settings, State, Window},
    sound::Sound,
    Future, Result,
};
use rand::Rng;
use std::process;

pub const GAME_AREA_WIDTH: usize = 8;
pub const GAME_AREA_HEIGHT: usize = 4;
pub const TILE_SIZE: i32 = 128;

const RESOLUTION_WIDTH: f32 = 1920f32;
const RESOLUTION_HEIGHT: f32 = 1080f32;
const VIEW_WIDTH: usize = 1440;
const VIEW_HEIGHT: usize = 810;

const SETTLEMENT_NUMBER_OF_LEVELS: usize = 13;
const SETTLEMENT_TEXTURE_FORMAT: &str = ".png";

const TILE_OWNER_CHANGE_PRICE: i32 = 5000;
const TILE_IMPROVEMENT_BASE_COST: i32 = 1000;
const TILE_POPULATION_CHANGE_BASE: i32 = 100;

struct BitterBoundaries {
    view: Rectangle,
    settlement_sprites: Vec<Asset<Image>>,
    sound_click: Asset<Sound>,
    sound_change: Asset<Sound>,
    sound_unable: Asset<Sound>,
    position: Vec<Vec<Vector>>,
    mouse_click_areas: Vec<Vec<Rectangle>>,
    menu_click_areas: Vec<Rectangle>,
    tile_owned_by: Vec<Vec<i32>>,
    tile_improvement_cost: Vec<Vec<i32>>,
    tile_improvement_level: Vec<Vec<i32>>,
    tile_population_number: Vec<Vec<i32>>,
    players_cash: [i32; 2],
    players_background_sprite: [Asset<Image>; 2],
    new_game_button_sprite: Asset<Image>,
    exit_button_sprite: Asset<Image>,
    back_to_main_menu_button: Asset<Image>,
    is_running: bool,
    is_win: bool,
    winner_player: usize,
}

impl State for BitterBoundaries {
    fn new() -> Result<BitterBoundaries> {
        let is_running: bool = false;
        let is_win: bool = false;
        let winner_player: usize = 2;

        let new_game_button_sprite: Asset<Image> =
            Asset::new(Image::load("sprites/gui/new_game_button.png"));
        let exit_button_sprite: Asset<Image> =
            Asset::new(Image::load("sprites/gui/exit_button.png"));
        let back_to_main_menu_button: Asset<Image> =
            Asset::new(Image::load("sprites/gui/back_to_main_menu_button.png"));

        let players_cash: [i32; 2] = [0, 0];
        let mut settlement_sprites = Vec::new();
        let mut menu_click_areas = Vec::new();
        let players_background_sprite: [Asset<Image>; 2] = [
            Asset::new(Image::load("sprites/terrains/red.png")),
            Asset::new(Image::load("sprites/terrains/blue.png")),
        ];
        for i in 0..SETTLEMENT_NUMBER_OF_LEVELS {
            let mut settlement_sprite_path: String = "sprites/settlements/level_".to_string();
            settlement_sprite_path.push_str(&(i.to_string()));
            settlement_sprite_path.push_str(SETTLEMENT_TEXTURE_FORMAT);
            settlement_sprites.push(Asset::new(Image::load(settlement_sprite_path)));
        }
        for i in 0..2 {
            menu_click_areas.push(Rectangle::new(
                Vector::new(
                    VIEW_WIDTH as i32 / 2 - TILE_SIZE,
                    VIEW_HEIGHT as i32 / 2 - TILE_SIZE / 2 + (i * 2) * TILE_SIZE,
                ),
                Vector::new(TILE_SIZE * 2, TILE_SIZE),
            ));
        }

        let sound_click = Asset::new(Sound::load("sounds/click.ogg"));
        let sound_change = Asset::new(Sound::load("sounds/change.ogg"));
        let sound_unable = Asset::new(Sound::load("sounds/unable.ogg"));

        let mut position = Vec::new();
        let mut mouse_click_areas = Vec::new();
        let mut tile_owned_by = Vec::new();
        let mut tile_improvement_cost = Vec::new();
        let mut tile_improvement_level = Vec::new();
        let mut tile_population_number = Vec::new();

        for i in 0..GAME_AREA_WIDTH {
            position.push(Vec::new());
            mouse_click_areas.push(Vec::new());

            tile_owned_by.push(Vec::new());
            tile_improvement_cost.push(Vec::new());
            tile_improvement_level.push(Vec::new());
            tile_population_number.push(Vec::new());

            for j in 0..GAME_AREA_HEIGHT {
                position[i].push(Vector::new(i as i32 * TILE_SIZE, j as i32 * TILE_SIZE));
                mouse_click_areas[i].push(Rectangle::new(
                    Vector::new(position[i][j].x as i32, position[i][j].y as i32),
                    Vector::new(TILE_SIZE, TILE_SIZE),
                ));
                if j < (GAME_AREA_HEIGHT / 2) {
                    tile_owned_by[i].push(0);
                } else {
                    tile_owned_by[i].push(1);
                }
                tile_improvement_cost[i].push(TILE_IMPROVEMENT_BASE_COST);
                tile_improvement_level[i].push(0);
                tile_population_number[i].push(1);
            }
        }

        let view = Rectangle::new_sized((VIEW_WIDTH as i32, VIEW_HEIGHT as i32));

        Ok(BitterBoundaries {
            sound_click,
            sound_change,
            sound_unable,
            view,
            players_background_sprite,
            settlement_sprites,
            position,
            mouse_click_areas,
            menu_click_areas,
            tile_owned_by,
            tile_improvement_cost,
            tile_improvement_level,
            tile_population_number,
            players_cash,
            new_game_button_sprite,
            exit_button_sprite,
            back_to_main_menu_button,
            is_running,
            is_win,
            winner_player,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if self.is_running {
            if self.is_win {
                if window.mouse()[MouseButton::Left] == ButtonState::Pressed
                    && self.menu_click_areas[0].contains(window.mouse().pos())
                {
                    self.is_running = false;
                }
            } else {
                for i in 0..self.players_cash.len() {
                    self.players_cash[i] +=
                        population_utility::get_cash(population_utility::get_total_population(
                            i as i32,
                            &self.tile_population_number,
                            &self.tile_owned_by,
                            GAME_AREA_WIDTH,
                            GAME_AREA_HEIGHT,
                        ));

                    if tile_utility::is_player_wins(
                        &self.tile_owned_by,
                        GAME_AREA_WIDTH,
                        GAME_AREA_HEIGHT,
                        i as i32,
                    ) {
                        self.is_win = true;
                        self.winner_player = i;
                    }
                }

                for i in 0..GAME_AREA_WIDTH {
                    for j in 0..GAME_AREA_HEIGHT {
                        if window.mouse()[MouseButton::Left] == ButtonState::Pressed
                            && self.mouse_click_areas[i][j].contains(window.mouse().pos())
                        {
                            if self.tile_owned_by[i][j] == 0 {
                                if self.players_cash[0] >= self.tile_improvement_cost[i][j] {
                                    self.players_cash[0] -= self.tile_improvement_cost[i][j];
                                    self.tile_improvement_cost[i][j] = TILE_IMPROVEMENT_BASE_COST
                                        * (self.tile_improvement_level[i][j] + 1);
                                    self.tile_population_number[i][j] += TILE_POPULATION_CHANGE_BASE
                                        * (self.tile_improvement_level[i][j] + 1);
                                    self.tile_improvement_level[i][j] =
                                        population_utility::get_level_of_settlement(
                                            self.tile_population_number[i][j],
                                        );
                                    self.sound_click.execute(|sound| {
                                        sound.play()?;
                                        Ok(())
                                    })?;
                                } else {
                                    self.sound_unable.execute(|sound| {
                                        sound.play()?;
                                        Ok(())
                                    })?;
                                }
                            } else if tile_utility::has_adjacent_friendly_tile(
                                &self.tile_owned_by,
                                i as i32,
                                j as i32,
                                GAME_AREA_WIDTH,
                                GAME_AREA_HEIGHT,
                                0,
                            ) {
                                let closest_settlement = tile_utility::closest_player_settlement(
                                    &self.tile_owned_by,
                                    i as i32,
                                    j as i32,
                                    GAME_AREA_WIDTH,
                                    GAME_AREA_HEIGHT,
                                    0,
                                );
                                if self.tile_population_number[i][j]
                                    >= TILE_POPULATION_CHANGE_BASE
                                        * (self.tile_improvement_level[i][j] + 1)
                                {
                                    self.tile_population_number[i][j] -= TILE_POPULATION_CHANGE_BASE
                                        * (self.tile_improvement_level[i][j] + 1);
                                    self.tile_population_number[closest_settlement.x]
                                        [closest_settlement.y] -= TILE_POPULATION_CHANGE_BASE
                                        * (self.tile_improvement_level[i][j] + 1);
                                    self.tile_improvement_level[i][j] =
                                        population_utility::get_level_of_settlement(
                                            self.tile_population_number[i][j],
                                        );
                                    self.tile_improvement_cost[i][j] = TILE_IMPROVEMENT_BASE_COST
                                        * (self.tile_improvement_level[i][j] + 1);
                                } else {
                                    if self.players_cash[0] >= TILE_OWNER_CHANGE_PRICE {
                                        self.players_cash[0] -= TILE_OWNER_CHANGE_PRICE;
                                        self.tile_owned_by[i][j] = 0;
                                        self.sound_change.execute(|sound| {
                                            sound.play()?;
                                            Ok(())
                                        })?;
                                    } else {
                                        self.sound_unable.execute(|sound| {
                                            sound.play()?;
                                            Ok(())
                                        })?;
                                    }
                                }
                            }
                        }
                    }
                }

                let mut rng = rand::thread_rng();
                let random_row: usize = rng.gen_range(0, GAME_AREA_HEIGHT);
                let random_column: usize = rng.gen_range(0, GAME_AREA_WIDTH);
                if self.tile_owned_by[random_column][random_row] == 1 {
                    if self.players_cash[1]
                        >= TILE_IMPROVEMENT_BASE_COST
                            * (self.tile_improvement_level[random_column][random_row] + 1)
                    {
                        self.players_cash[1] -= TILE_IMPROVEMENT_BASE_COST
                            * (self.tile_improvement_level[random_column][random_row] + 1);
                        self.tile_population_number[random_column][random_row] +=
                            TILE_POPULATION_CHANGE_BASE
                                * (self.tile_improvement_level[random_column][random_row] + 1);
                        self.tile_improvement_level[random_column][random_row] =
                            population_utility::get_level_of_settlement(
                                self.tile_population_number[random_column][random_row],
                            );
                        self.sound_click.execute(|sound| {
                            sound.play()?;
                            Ok(())
                        })?;
                    }
                } else {
                    let closest_settlement = tile_utility::closest_player_settlement(
                        &self.tile_owned_by,
                        random_column as i32,
                        random_row as i32,
                        GAME_AREA_WIDTH,
                        GAME_AREA_HEIGHT,
                        1,
                    );
                    if self.tile_population_number[random_column][random_row]
                        > TILE_POPULATION_CHANGE_BASE
                            * (self.tile_improvement_level[random_column][random_row] + 1)
                    {
                        self.tile_population_number[random_column][random_row] -=
                            TILE_POPULATION_CHANGE_BASE
                                * (self.tile_improvement_level[random_column][random_row] + 1);
                        self.tile_population_number[closest_settlement.x][closest_settlement.y] -=
                            TILE_POPULATION_CHANGE_BASE
                                * (self.tile_improvement_level[random_column][random_row] + 1);
                        self.tile_improvement_level[random_column][random_row] =
                            population_utility::get_level_of_settlement(
                                self.tile_population_number[random_column][random_row],
                            );
                        self.tile_improvement_cost[random_column][random_row] =
                            TILE_IMPROVEMENT_BASE_COST
                                * (self.tile_improvement_level[random_column][random_row] + 1);
                    } else {
                        if self.players_cash[1] >= TILE_OWNER_CHANGE_PRICE {
                            self.players_cash[1] -= TILE_OWNER_CHANGE_PRICE;
                            self.tile_owned_by[random_column][random_row] = 1;
                            self.sound_change.execute(|sound| {
                                sound.play()?;
                                Ok(())
                            })?;
                        }
                    }
                }
            }
        } else {
            if window.mouse()[MouseButton::Left] == ButtonState::Pressed
                && self.menu_click_areas[0].contains(window.mouse().pos())
            {
                self.is_running = true;
                self.is_win = false;
                self.winner_player = 2;
            }

            if window.mouse()[MouseButton::Left] == ButtonState::Pressed
                && self.menu_click_areas[1].contains(window.mouse().pos())
            {
                process::exit(0x0100);
            }
        }

        if window.keyboard()[Key::Left].is_down() {
            if self.view.pos.x > (0.0f32 - 0.1f32 * self.view.size.x) {
                self.view = self.view.translate((-4, 0));
            }
        }
        if window.keyboard()[Key::Right].is_down() {
            if self.view.pos.x < (RESOLUTION_WIDTH - 0.9f32 * self.view.size.x) {
                self.view = self.view.translate((4, 0));
            }
        }
        if window.keyboard()[Key::Down].is_down() {
            if self.view.pos.y < (RESOLUTION_HEIGHT - 0.9f32 * self.view.size.y) {
                self.view = self.view.translate((0, 4));
            }
        }
        if window.keyboard()[Key::Up].is_down() {
            if self.view.pos.y > (0.0f32 - 0.1f32 * self.view.size.y) {
                self.view = self.view.translate((0, -4));
            }
        }
        window.set_view(View::new(self.view));

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;

        let fontstyle_white_9: FontStyle = FontStyle::new(9.0, Color::WHITE);
        let fontstyle_white_12: FontStyle = FontStyle::new(12.0, Color::WHITE);

        if self.is_running {
            if self.is_win {
                let winner_string: String =
                    "Winner: Player ".to_string() + &(self.winner_player.to_string());
                let mut winner_text: Asset<Image> = Asset::new(
                    Font::load("fonts/FiraCode-Regular.ttf").and_then(move |font| {
                        result(font.render(&winner_string, &fontstyle_white_12))
                    }),
                );
                winner_text.execute(|image| {
                    window.draw(
                        &image.area().with_center((
                            GAME_AREA_WIDTH as i32 * TILE_SIZE + TILE_SIZE / 2,
                            GAME_AREA_HEIGHT as i32 * TILE_SIZE + 18,
                        )),
                        Img(&image),
                    );
                    Ok(())
                })?;
                self.back_to_main_menu_button.execute(|image| {
                    window.draw(
                        &image
                            .area()
                            .with_center((VIEW_WIDTH as i32 / 2, VIEW_HEIGHT as i32 / 2)),
                        Img(&image),
                    );
                    Ok(())
                })?;
            } else {
                let players_cash_string: String =
                    "Cash: ".to_string() + &(self.players_cash[0].to_string());
                let mut players_cash_text: Asset<Image> = Asset::new(
                    Font::load("fonts/FiraCode-Regular.ttf").and_then(move |font| {
                        result(font.render(&players_cash_string, &fontstyle_white_12))
                    }),
                );
                players_cash_text.execute(|image| {
                    window.draw(
                        &image.area().with_center((
                            GAME_AREA_WIDTH as i32 * TILE_SIZE + TILE_SIZE / 2,
                            GAME_AREA_HEIGHT as i32 * TILE_SIZE + 18,
                        )),
                        Img(&image),
                    );
                    Ok(())
                })?;

                for i in 0..GAME_AREA_WIDTH {
                    for j in 0..GAME_AREA_HEIGHT {
                        let population_number_string: String =
                            String::from(self.tile_population_number[i][j].to_string());
                        let settlement_type_string: String =
                            population_utility::get_type_of_settlement(
                                self.tile_population_number[i][j],
                            );
                        let improvement_cost_string: String =
                            self.tile_improvement_cost[i][j].to_string();

                        let mut population_number_text: Asset<Image> = Asset::new(
                            Font::load("fonts/FiraCode-Regular.ttf").and_then(move |font| {
                                result(font.render(&population_number_string, &fontstyle_white_12))
                            }),
                        );

                        let mut settlement_type_text: Asset<Image> = Asset::new(
                            Font::load("fonts/FiraCode-Regular.ttf").and_then(move |font| {
                                result(font.render(&settlement_type_string, &fontstyle_white_9))
                            }),
                        );

                        let mut improvement_cost_text: Asset<Image> = Asset::new(
                            Font::load("fonts/FiraCode-Regular.ttf").and_then(move |font| {
                                result(font.render(&improvement_cost_string, &fontstyle_white_12))
                            }),
                        );

                        population_number_text.execute(|image| {
                            window.draw(
                                &image.area().with_center((
                                    i as i32 * TILE_SIZE + TILE_SIZE / 2,
                                    j as i32 * TILE_SIZE + 6,
                                )),
                                Img(&image),
                            );
                            Ok(())
                        })?;

                        settlement_type_text.execute(|image| {
                            window.draw(
                                &image.area().with_center((
                                    i as i32 * TILE_SIZE + TILE_SIZE / 2,
                                    j as i32 * TILE_SIZE + 18,
                                )),
                                Img(&image),
                            );
                            Ok(())
                        })?;

                        improvement_cost_text.execute(|image| {
                            window.draw(
                                &image.area().with_center((
                                    i as i32 * TILE_SIZE + TILE_SIZE / 2,
                                    j as i32 * TILE_SIZE + TILE_SIZE - 20,
                                )),
                                Img(&image),
                            );
                            Ok(())
                        })?;

                        let new_x: i32 = self.position[i][j].x as i32;
                        let new_y: i32 = self.position[i][j].y as i32;

                        self.players_background_sprite[self.tile_owned_by[i][j] as usize].execute(
                            |image| {
                                window.draw(
                                    &image.area().with_center((
                                        TILE_SIZE / 2 + new_x,
                                        TILE_SIZE / 2 + new_y,
                                    )),
                                    Img(&image),
                                );
                                Ok(())
                            },
                        )?;
                    }
                }

                for i in 0..GAME_AREA_WIDTH {
                    for j in 0..GAME_AREA_HEIGHT {
                        let new_x: i32 = self.position[i][j].x as i32;
                        let new_y: i32 = self.position[i][j].y as i32;

                        self.settlement_sprites[self.tile_improvement_level[i][j] as usize]
                            .execute(|image| {
                                window.draw(
                                    &image.area().with_center((
                                        TILE_SIZE / 2 + new_x,
                                        TILE_SIZE / 2 + new_y,
                                    )),
                                    Img(&image),
                                );
                                Ok(())
                            })?;
                    }
                }
            }
        } else {
            self.new_game_button_sprite.execute(|image| {
                window.draw(
                    &image
                        .area()
                        .with_center((VIEW_WIDTH as i32 / 2, VIEW_HEIGHT as i32 / 2)),
                    Img(&image),
                );
                Ok(())
            })?;
            self.exit_button_sprite.execute(|image| {
                window.draw(
                    &image.area().with_center((
                        VIEW_WIDTH as i32 / 2,
                        VIEW_HEIGHT as i32 / 2 + 2 * TILE_SIZE,
                    )),
                    Img(&image),
                );
                Ok(())
            })?;
        }

        Ok(())
    }
}

fn main() {
    std::env::set_var("WINIT_HIDPI_FACTOR", "1.0");
    run::<BitterBoundaries>(
        "Bitter Boundaries",
        Vector::new(1920, 1080),
        Settings {
            draw_rate: 33.33,
            icon_path: Some("sprites/settlements/level_12.png"),
            scale: quicksilver::graphics::ImageScaleStrategy::Blur,
            vsync: false,
            ..Settings::default()
        },
    );
}
