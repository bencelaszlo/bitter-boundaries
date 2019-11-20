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

pub const GAME_AREA_WIDTH: usize = 8;
pub const GAME_AREA_HEIGHT: usize = 4;
pub const TILE_SIZE: i32 = 128;

const RESOLUTION_WIDTH: f32 = 1920f32;
const RESOLUTION_HEIGHT: f32 = 1080f32;

const SETTLEMENT_NUMBER_OF_LEVELS: usize = 13;
const SETTLEMENT_TEXTURE_FORMAT: &str = ".png";

struct BitterBoundaries {
    view: Rectangle,
    resolution_width: f32,
    resolution_height: f32,
    settlement_sprites: Vec<Asset<Image>>,
    sound_click: Asset<Sound>,
    sound_change: Asset<Sound>,
    sound_unable: Asset<Sound>,
    position: Vec<Vec<Vector>>,
    mouse_click_areas: Vec<Vec<Rectangle>>,
    tile_owned_by: Vec<Vec<i32>>,
    tile_improvement_level: Vec<Vec<i32>>,
    tile_population_number: Vec<Vec<i32>>,
    players_cash: Vec<i32>,
    players_background_sprite: Vec<Asset<Image>>,
}

impl State for BitterBoundaries {
    fn new() -> Result<BitterBoundaries> {
        let mut settlement_sprites = Vec::new();
        let mut players_cash = Vec::new();
        players_cash.push(0);
        players_cash.push(0);
        let mut players_background_sprite = Vec::new();
        players_background_sprite.push(Asset::new(Image::load("sprites/terrains/red.png")));
        players_background_sprite.push(Asset::new(Image::load("sprites/terrains/blue.png")));
        for i in 0..SETTLEMENT_NUMBER_OF_LEVELS {
            let mut settlement_sprite_path: String = "sprites/settlements/level_".to_string();
            settlement_sprite_path.push_str(&(i.to_string()));
            settlement_sprite_path.push_str(SETTLEMENT_TEXTURE_FORMAT);
            settlement_sprites.push(Asset::new(Image::load(settlement_sprite_path)));
        }

        let sound_click = Asset::new(Sound::load("sounds/click.ogg"));
        let sound_change = Asset::new(Sound::load("sounds/change.ogg"));
        let sound_unable = Asset::new(Sound::load("sounds/unable.ogg"));

        let mut position = Vec::new();
        let mut mouse_click_areas = Vec::new();
        let mut tile_owned_by = Vec::new();
        let mut tile_improvement_level = Vec::new();
        let mut tile_population_number = Vec::new();

        for i in 0..GAME_AREA_WIDTH {
            position.push(Vec::new());
            mouse_click_areas.push(Vec::new());

            tile_owned_by.push(Vec::new());
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
                tile_improvement_level[i].push(0);
                tile_population_number[i].push(1);
            }
        }

        Ok(BitterBoundaries {
            sound_click,
            sound_change,
            sound_unable,
            view: Rectangle::new_sized((1440, 810)),
            resolution_width: RESOLUTION_WIDTH,
            resolution_height: RESOLUTION_HEIGHT,
            players_background_sprite,
            settlement_sprites,
            position,
            mouse_click_areas,
            tile_owned_by,
            tile_improvement_level,
            tile_population_number,
            players_cash,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
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
                println!("player {} is the winner!", i);
            }
        }

        for i in 0..GAME_AREA_WIDTH {
            for j in 0..GAME_AREA_HEIGHT {
                if window.mouse()[MouseButton::Right] == ButtonState::Pressed
                    && self.mouse_click_areas[i][j].contains(window.mouse().pos())
                {
                    self.tile_owned_by[i][j] = !self.tile_owned_by[i][j];
                }

                if window.mouse()[MouseButton::Left] == ButtonState::Pressed
                    && self.mouse_click_areas[i][j].contains(window.mouse().pos())
                {
                    if self.tile_owned_by[i][j] == 0 {
                        if self.players_cash[0] >= 1000 * (self.tile_improvement_level[i][j] + 1) {
                            self.players_cash[0] -= 1000 * (self.tile_improvement_level[i][j] + 1);
                            self.tile_population_number[i][j] +=
                                100 * (self.tile_improvement_level[i][j] + 1);
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
                    } else {
                        let closest_settlement = tile_utility::closest_player_settlement(
                            &self.tile_population_number,
                            i as i32,
                            j as i32,
                            GAME_AREA_WIDTH,
                            GAME_AREA_HEIGHT,
                        );
                        if self.tile_population_number[i][j]
                            > 1000 * (self.tile_improvement_level[i][j] + 1)
                        {
                            self.tile_population_number[i][j] -=
                                100 * (self.tile_improvement_level[i][j] + 1);
                            self.tile_population_number[closest_settlement.x]
                                [closest_settlement.y] -=
                                100 * (self.tile_improvement_level[i][j] + 1);
                            self.tile_improvement_level[i][j] =
                                population_utility::get_level_of_settlement(
                                    self.tile_population_number[i][j],
                                );
                        } else {
                            if self.players_cash[0] >= 10000 {
                                self.players_cash[0] -= 10000;
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
                >= 1000 * (self.tile_improvement_level[random_column][random_row] + 1)
            {
                self.players_cash[1] -=
                    1000 * (self.tile_improvement_level[random_column][random_row] + 1);
                self.tile_population_number[random_column][random_row] +=
                    100 * (self.tile_improvement_level[random_column][random_row] + 1);
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
                &self.tile_population_number,
                random_column as i32,
                random_row as i32,
                GAME_AREA_WIDTH,
                GAME_AREA_HEIGHT,
            );
            if self.tile_population_number[random_column][random_row]
                > 1000 * (self.tile_improvement_level[random_column][random_row] + 1)
            {
                self.tile_population_number[random_column][random_row] -=
                    100 * (self.tile_improvement_level[random_column][random_row] + 1);
                self.tile_population_number[closest_settlement.x][closest_settlement.y] -=
                    100 * (self.tile_improvement_level[random_column][random_row] + 1);
                self.tile_improvement_level[random_column][random_row] =
                    population_utility::get_level_of_settlement(
                        self.tile_population_number[random_column][random_row],
                    );
            } else {
                if self.players_cash[1] >= 10000 {
                    self.players_cash[1] -= 10000;
                    self.tile_owned_by[random_column][random_row] = 1;
                    self.sound_change.execute(|sound| {
                        sound.play()?;
                        Ok(())
                    })?;
                }
            }
        }

        if window.keyboard()[Key::Left].is_down() {
            if self.view.pos.x > (0.0f32 - 0.1f32 * self.view.size.x) {
                self.view = self.view.translate((-4, 0));
            }
        }
        if window.keyboard()[Key::Right].is_down() {
            if self.view.pos.x < (self.resolution_width - 0.9f32 * self.view.size.x) {
                self.view = self.view.translate((4, 0));
            }
        }
        if window.keyboard()[Key::Down].is_down() {
            if self.view.pos.y < (self.resolution_height - 0.9f32 * self.view.size.y) {
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
                let settlement_type: String =
                    population_utility::get_type_of_settlement(self.tile_population_number[i][j]);

                let mut population_number_text: Asset<Image> = Asset::new(
                    Font::load("fonts/FiraCode-Regular.ttf").and_then(move |font| {
                        result(font.render(&population_number_string, &fontstyle_white_12))
                    }),
                );

                let mut settlement_type_text: Asset<Image> = Asset::new(
                    Font::load("fonts/FiraCode-Regular.ttf").and_then(move |font| {
                        result(font.render(&settlement_type, &fontstyle_white_9))
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

                let new_x: i32 = self.position[i][j].x as i32;
                let new_y: i32 = self.position[i][j].y as i32;

                self.players_background_sprite[self.tile_owned_by[i][j] as usize].execute(
                    |image| {
                        window.draw(
                            &image
                                .area()
                                .with_center((TILE_SIZE / 2 + new_x, TILE_SIZE / 2 + new_y)),
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

                self.settlement_sprites[self.tile_improvement_level[i][j] as usize].execute(
                    |image| {
                        window.draw(
                            &image
                                .area()
                                .with_center((TILE_SIZE / 2 + new_x, TILE_SIZE / 2 + new_y)),
                            Img(&image),
                        );
                        Ok(())
                    },
                )?;
            }
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
            draw_rate: 16.67,
            icon_path: Some("sprites/settlements/level_12.png"),
            scale: quicksilver::graphics::ImageScaleStrategy::Blur,
            vsync: false,
            ..Settings::default()
        },
    );
}
