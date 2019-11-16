mod population_utility;
mod tile_utility;

extern crate image;
extern crate quicksilver;
extern crate rand;

use rand::Rng;

use quicksilver::{
    combinators::result,
    geom::{Rectangle, Shape, Vector},
    graphics::{Background::Img, Color, Font, FontStyle, Image, View},
    input::{ButtonState, Key, MouseButton},
    lifecycle::{run, Asset, Settings, State, Window},
    Future,
    Result,
    // sound::Sound,
};

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
    red_sprite: Asset<Image>,
    blue_sprite: Asset<Image>,
    settlement_sprites: Vec<Asset<Image>>,
    // sound: Asset<Sound>,
    position: Vec<Vec<Vector>>,
    mouse_click_areas: Vec<Vec<Rectangle>>,
    tile_owned_by: Vec<Vec<i32>>,
    tile_improvement_level: Vec<Vec<i32>>,
    tile_population_number: Vec<Vec<i32>>,
    players_cash: Vec<i32>,
}

/* const BUTTON_AREA: Rectangle = Rectangle {
    pos: Vector { x: 350.0, y: 250.0 },
    size: Vector { x: 100.0, y: 100.0 },
}; */

impl State for BitterBoundaries {
    fn new() -> Result<BitterBoundaries> {
        let red_sprite = Asset::new(Image::load("sprites/terrains/red.png"));
        let blue_sprite = Asset::new(Image::load("sprites/terrains/blue.png"));
        let mut settlement_sprites = Vec::new();
        let mut players_cash = Vec::new();
        players_cash.push(0);
        players_cash.push(0);
        for i in 0..SETTLEMENT_NUMBER_OF_LEVELS {
            let mut settlement_sprite_path: String = "sprites/settlements/level_".to_string();
            settlement_sprite_path.push_str(&(i.to_string()));
            settlement_sprite_path.push_str(SETTLEMENT_TEXTURE_FORMAT);
            settlement_sprites.push(Asset::new(Image::load(settlement_sprite_path)));
        }

        // let sound = Asset::new(Sound::load("sounds/test_sound.ogg"));

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
                tile_improvement_level[i].push(-1);
                tile_population_number[i].push(0);
            }
        }

        Ok(BitterBoundaries {
            view: Rectangle::new_sized((1440, 810)),
            resolution_width: RESOLUTION_WIDTH,
            resolution_height: RESOLUTION_HEIGHT,
            red_sprite,
            blue_sprite,
            settlement_sprites,
            // sound,
            position,
            mouse_click_areas,
            tile_owned_by,
            tile_improvement_level,
            tile_population_number,
            players_cash,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.players_cash[0] += 50;
        self.players_cash[1] += 50;

        /* if window.mouse()[MouseButton::Left] == ButtonState::Pressed
            && BUTTON_AREA.contains(window.mouse().pos())
        {
            self.sound.execute(|sound| {
                sound.play()?;
                Ok(())
            })?;
        } */

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
                        if self.players_cash[0] >= 1000 {
                            self.players_cash[0] -= 1000;
                            self.tile_population_number[i][j] += 100;
                            self.tile_improvement_level[i][j] =
                                population_utility::get_level_of_settlement(
                                    self.tile_population_number[i][j],
                                );
                        }
                    } else {
                        let closest_settlement = tile_utility::closest_player_settlement(
                            &self.tile_population_number,
                            i as i32,
                            j as i32,
                            GAME_AREA_WIDTH,
                            GAME_AREA_HEIGHT,
                        );
                        if self.tile_population_number[i][j] > 100 {
                            self.tile_population_number[i][j] -= 100;
                            self.tile_population_number[closest_settlement.x]
                                [closest_settlement.y] -= 100;
                            self.tile_improvement_level[i][j] =
                                population_utility::get_level_of_settlement(
                                    self.tile_population_number[i][j],
                                );
                        } else {
                            self.tile_owned_by[i][j] = 0;
                        }
                    }
                }
            }
        }

        let mut rng = rand::thread_rng();
        let random_row: usize = rng.gen_range(GAME_AREA_HEIGHT / 2, GAME_AREA_HEIGHT);
        let random_column: usize = rng.gen_range(0, GAME_AREA_WIDTH);
        if self.players_cash[1] >= 1000 {
            self.players_cash[1] -= 1000;
            self.tile_population_number[random_column][random_row] += 100;
            self.tile_improvement_level[random_column][random_row] =
                population_utility::get_level_of_settlement(
                    self.tile_population_number[random_column][random_row],
                );
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

        let fontstyle_black_9: FontStyle = FontStyle::new(9.0, Color::BLACK);
        let fontstyle_white_12: FontStyle = FontStyle::new(12.0, Color::WHITE);

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
                        result(font.render(&settlement_type, &fontstyle_black_9))
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
            }
        }

        // If the sound is loaded, draw the button
        /* self.sound.execute(|_| {
            window.draw(&BUTTON_AREA, Col(Color::BLUE));
            Ok(())
        })?; */

        for i in 0..GAME_AREA_WIDTH {
            for j in 0..GAME_AREA_HEIGHT {
                let new_x = self.position[i][j].x;
                let new_y = self.position[i][j].y;
                if self.tile_owned_by[i][j] == 0 {
                    self.red_sprite.execute(|image| {
                        window.draw(
                            &image.area().with_center((
                                TILE_SIZE / 2 + new_x as i32,
                                TILE_SIZE / 2 + new_y as i32,
                            )),
                            Img(&image),
                        );
                        Ok(())
                    })?
                } else {
                    self.blue_sprite.execute(|image| {
                        window.draw(
                            &image.area().with_center((
                                TILE_SIZE / 2 + new_x as i32,
                                TILE_SIZE / 2 + new_y as i32,
                            )),
                            Img(&image),
                        );
                        Ok(())
                    })?
                }
                match self.tile_improvement_level[i][j] {
                    0 => self.settlement_sprites[0].execute(|image| {
                        window.draw(
                            &image.area().with_center((
                                TILE_SIZE / 2 + new_x as i32,
                                TILE_SIZE / 2 + new_y as i32,
                            )),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    1 => self.settlement_sprites[1].execute(|image| {
                        window.draw(
                            &image.area().with_center((
                                TILE_SIZE / 2 + new_x as i32,
                                TILE_SIZE / 2 + new_y as i32,
                            )),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    2 => self.settlement_sprites[2].execute(|image| {
                        window.draw(
                            &image.area().with_center((
                                TILE_SIZE / 2 + new_x as i32,
                                TILE_SIZE / 2 + new_y as i32,
                            )),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    3 => self.settlement_sprites[3].execute(|image| {
                        window.draw(
                            &image.area().with_center((
                                TILE_SIZE / 2 + new_x as i32,
                                TILE_SIZE / 2 + new_y as i32,
                            )),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    4 => self.settlement_sprites[4].execute(|image| {
                        window.draw(
                            &image.area().with_center((
                                TILE_SIZE / 2 + new_x as i32,
                                TILE_SIZE / 2 + new_y as i32,
                            )),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    5 => self.settlement_sprites[5].execute(|image| {
                        window.draw(
                            &image.area().with_center((
                                TILE_SIZE / 2 + new_x as i32,
                                TILE_SIZE / 2 + new_y as i32,
                            )),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    6 => self.settlement_sprites[6].execute(|image| {
                        window.draw(
                            &image.area().with_center((
                                TILE_SIZE / 2 + new_x as i32,
                                TILE_SIZE / 2 + new_y as i32,
                            )),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    7 => self.settlement_sprites[7].execute(|image| {
                        window.draw(
                            &image.area().with_center((
                                TILE_SIZE / 2 + new_x as i32,
                                TILE_SIZE / 2 + new_y as i32,
                            )),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    8 => self.settlement_sprites[8].execute(|image| {
                        window.draw(
                            &image.area().with_center((
                                TILE_SIZE / 2 + new_x as i32,
                                TILE_SIZE / 2 + new_y as i32,
                            )),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    9 => self.settlement_sprites[9].execute(|image| {
                        window.draw(
                            &image.area().with_center((
                                TILE_SIZE / 2 + new_x as i32,
                                TILE_SIZE / 2 + new_y as i32,
                            )),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    10 => self.settlement_sprites[10].execute(|image| {
                        window.draw(
                            &image.area().with_center((
                                TILE_SIZE / 2 + new_x as i32,
                                TILE_SIZE / 2 + new_y as i32,
                            )),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    11 => self.settlement_sprites[11].execute(|image| {
                        window.draw(
                            &image.area().with_center((
                                TILE_SIZE / 2 + new_x as i32,
                                TILE_SIZE / 2 + new_y as i32,
                            )),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    _ => self.settlement_sprites[12].execute(|image| {
                        window.draw(
                            &image.area().with_center((
                                TILE_SIZE / 2 + new_x as i32,
                                TILE_SIZE / 2 + new_y as i32,
                            )),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                }
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
            icon_path: Some("sprites/settlements/level_12.png"),
            scale: quicksilver::graphics::ImageScaleStrategy::Blur,
            ..Settings::default()
        },
    );
}
