extern crate image;
extern crate quicksilver;

use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Background::Col, Background::Img, Color, Image, View},
    input::{ButtonState, Key, MouseButton},
    lifecycle::{run, Asset, Settings, State, Window},
    sound::Sound,
    Result,
};

struct BitterBoundaries {
    view: Rectangle,
    resolution_width: f32,
    resolution_height: f32,
    sand_sprite: Asset<Image>,
    hill_sprite: Asset<Image>,
    mountain_sprite: Asset<Image>,
    water_sprite: Asset<Image>,
    // industrial_sprites: Vec<Asset<Image>>,
    sound: Asset<Sound>,
    position: Vec<Vec<Vector>>,
    mouse_click_areas: Vec<Vec<Rectangle>>,
    tile_type: Vec<Vec<i32>>,
    tile_improvement_level: Vec<Vec<i32>>,
}

const BUTTON_AREA: Rectangle = Rectangle {
    pos: Vector { x: 350.0, y: 250.0 },
    size: Vector { x: 100.0, y: 100.0 },
};

impl State for BitterBoundaries {
    fn new() -> Result<BitterBoundaries> {
        let sand_sprite = Asset::new(Image::load("sprites/terrains/sand.png"));
        let hill_sprite = Asset::new(Image::load("sprites/terrains/hill.png"));
        let mountain_sprite = Asset::new(Image::load("sprites/terrains/mountain.png"));
        let water_sprite = Asset::new(Image::load("sprites/terrains/water.png"));
        /* let mut industrial_sprites = Vec::new();
        for i in 0..6 {
            let mut industrial_sprite_path: String =
                "sprites/industrial/industrial_level".to_string();
            industrial_sprite_path.push_str(&(i.to_string()));
            industrial_sprite_path.push_str(".png");
            industrial_sprites.push(Asset::new(Image::load(industrial_sprite_path)));
        } */

        let sound = Asset::new(Sound::load("sounds/test_sound.ogg"));

        let mut position = Vec::new();
        let mut mouse_click_areas = Vec::new();
        let mut tile_type = Vec::new();
        let mut tile_improvement_level = Vec::new();
        for i in 0..32 {
            position.push(Vec::new());
            mouse_click_areas.push(Vec::new());
            tile_type.push(Vec::new());
            tile_improvement_level.push(Vec::new());
            for j in 0..16 {
                position[i].push(Vector::new(i as i32 * 60, j as i32 * 60));
                mouse_click_areas[i].push(Rectangle::new(
                    Vector::new(position[i][j].x as i32, position[i][j].y as i32),
                    Vector::new(60, 60),
                ));
                tile_type[i].push(0);
                tile_improvement_level[i].push(-1);
            }
        }

        Ok(BitterBoundaries {
            view: Rectangle::new_sized((400, 300)),
            resolution_width: 1920f32,
            resolution_height: 1080f32,
            sand_sprite,
            hill_sprite,
            mountain_sprite,
            water_sprite,
            sound,
            position,
            mouse_click_areas,
            tile_type,
            tile_improvement_level,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if window.mouse()[MouseButton::Left] == ButtonState::Pressed
            && BUTTON_AREA.contains(window.mouse().pos())
        {
            self.sound.execute(|sound| {
                sound.play()?;
                Ok(())
            })?;
        }

        for i in 0..32 {
            for j in 0..16 {
                if window.mouse()[MouseButton::Right] == ButtonState::Pressed
                    && self.mouse_click_areas[i][j].contains(window.mouse().pos())
                {
                    self.tile_type[i][j] += 1;
                }
                if window.mouse()[MouseButton::Left] == ButtonState::Pressed
                    && self.mouse_click_areas[i][j].contains(window.mouse().pos())
                {
                    self.tile_improvement_level[i][j] += 1;
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
        window.clear(Color::WHITE)?;

        // If the sound is loaded, draw the button
        self.sound.execute(|_| {
            window.draw(&BUTTON_AREA, Col(Color::BLUE));
            Ok(())
        })?;

        for i in 0..32 {
            for j in 0..16 {
                let new_x = self.position[i][j].x;
                let new_y = self.position[i][j].y;
                match self.tile_type[i][j] {
                    0 => self.sand_sprite.execute(|image| {
                        window.draw(
                            &image
                                .area()
                                .with_center((30 + new_x as i32, 30 + new_y as i32)),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    1 => self.hill_sprite.execute(|image| {
                        window.draw(
                            &image
                                .area()
                                .with_center((30 + new_x as i32, 30 + new_y as i32)),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    2 => self.mountain_sprite.execute(|image| {
                        window.draw(
                            &image
                                .area()
                                .with_center((30 + new_x as i32, 30 + new_y as i32)),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    3 => self.water_sprite.execute(|image| {
                        window.draw(
                            &image
                                .area()
                                .with_center((30 + new_x as i32, 30 + new_y as i32)),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    _ => println!("Unsupported tile type at {} - {}.", i, j),
                }
                /* match self.tile_improvement_level[i][j] {
                    0 => self.industrial_sprites[0].execute(|image| {
                        window.draw(
                            &image
                                .area()
                                .with_center((30 + new_x as i32, 30 + new_y as i32)),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    1 => self.industrial_sprites[1].execute(|image| {
                        window.draw(
                            &image
                                .area()
                                .with_center((30 + new_x as i32, 30 + new_y as i32)),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    2 => self.industrial_sprites[2].execute(|image| {
                        window.draw(
                            &image
                                .area()
                                .with_center((30 + new_x as i32, 30 + new_y as i32)),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    3 => self.industrial_sprites[3].execute(|image| {
                        window.draw(
                            &image
                                .area()
                                .with_center((30 + new_x as i32, 30 + new_y as i32)),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    4 => self.industrial_sprites[4].execute(|image| {
                        window.draw(
                            &image
                                .area()
                                .with_center((30 + new_x as i32, 30 + new_y as i32)),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    5 => self.industrial_sprites[5].execute(|image| {
                        window.draw(
                            &image
                                .area()
                                .with_center((30 + new_x as i32, 30 + new_y as i32)),
                            Img(&image),
                        );
                        Ok(())
                    })?,
                    _ => println!("Unsupported tile improvement level at {} - {}", i, j),
                } */
            }
        }
        Ok(())
    }
}

fn get_type_of_settlement(number_of_population: i32) -> String {
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

fn main() {
    run::<BitterBoundaries>(
        "Bitter Boundaries",
        Vector::new(1920, 1080),
        Settings {
            // icon_path: Some("sprites/wall/wall_level5.png"),
            ..Settings::default()
        },
    );
}
