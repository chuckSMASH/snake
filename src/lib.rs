extern crate piston;
extern crate rand;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod objects;


use std::cmp::min;
use std::path::Path;

use graphics::{ clear, ellipse, rectangle, Text, Transformed };
use graphics::color::hex;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event_loop::{ Events, EventLoop, EventSettings };
use piston::input::{ Button, RenderEvent, PressEvent, Input };
use piston::input::keyboard::Key;
use piston::window::{ Size, Window as PistonWindow, WindowSettings };

use objects::Food;
use objects::Direction;
use objects::Grid;
use objects::Snake;


pub struct Game<'a> {
    gl: GlGraphics,
    font_cache: GlyphCache<'a>,
    pending_direction: Option<Direction>,
    game_over: bool,
    snake: Snake,
    food: Food,
    grid: Grid,
    score: u32,
}


impl<'a> Game<'a> {
    fn on_press(&mut self, e: &Input) {
        if self.game_over {
            return
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up if self.snake.direction() != Direction::Down => {
                    self.pending_direction = Some(Direction::Up);
                },
                Key::Right if self.snake.direction() != Direction::Left => {
                    self.pending_direction = Some(Direction::Right);
                },
                Key::Down if self.snake.direction() != Direction::Up => {
                    self.pending_direction = Some(Direction::Down);
                }
                Key::Left if self.snake.direction() != Direction::Right => {
                    self.pending_direction = Some(Direction::Left);
                },
                _ => {}
            }
        }
    }

    fn on_update(&mut self) {
        if self.game_over {
            self.on_game_over();
            return;
        }
        if let Some(direction) = self.pending_direction {
            self.pending_direction = None;
            self.snake.set_direction(direction);
        };
        let moved = self.snake.traverse(&self.grid);
        if !moved {
            self.game_over = true;
        }
        if self.snake.eats(&self.food) {
            self.score += self.food.points();
            self.food = Food::generate(&self.grid, &self.snake);
        }
    }

    fn on_render(&mut self, e: &Input) {
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const SQUARE_SIZE: f64 = 30.0;
        const FOOD_SIZE: f64 = 20.0;

        let args = e.render_args().unwrap();
        let squares = self.snake.squares().iter();
        let food = &self.food;
        let snake_color = hex("3c53a0");
        let score = format!("{: >4}", self.score);
        let ref mut font_cache = self.font_cache;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            rectangle(snake_color, [200.0, 270.0, 600.0, 30.0], c.transform, gl);
            rectangle(WHITE, [200.0, 300.0, 600.0, 450.0], c.transform, gl);

            let text_trans = c.transform.trans(750.0, 295.0);
            let text = Text::new_color(WHITE, 20);
            text.draw(&score, font_cache, &c.draw_state,
                      text_trans, gl);



            let food_tlx = (food.get_square().x() as f64) * SQUARE_SIZE + 205.0;
            let food_tly = (food.get_square().y() as f64) * SQUARE_SIZE + 305.0;
            ellipse(snake_color, [food_tlx, food_tly, FOOD_SIZE, FOOD_SIZE],
                    c.transform, gl);

            for square in squares {
                let top_left_x = (square.x() as f64) * SQUARE_SIZE + 200.0;
                let top_left_y = (square.y() as f64) * SQUARE_SIZE + 300.0;
                rectangle(snake_color, [top_left_x, top_left_y, SQUARE_SIZE, SQUARE_SIZE],
                          c.transform, gl);
            }
        });
    }

    pub fn on_game_over(&mut self) {
        if self.score > 0 {
            self.score -= min(2, self.score);
        }
        let drained = self.snake.drain();
        if self.score == 0 && drained {
            self.snake.set_direction(Direction::NoDirection);
            self.game_over = false;
        }
    }

    pub fn run() {
        let opengl = OpenGL::V3_2;
        let mut window: Window = WindowSettings::new(
            "snake",
            [1000, 1000])
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let font_path = Path::new("assets/Verdana.ttf");
        let grid = Grid::new(20, 15);
        let snake = Snake::new(10, 10);
        let food = Food::generate(&grid, &snake);
        let font_cache = GlyphCache::new(font_path).unwrap();
        let mut game = Game {
            gl: GlGraphics::new(opengl),
            font_cache: font_cache,
            pending_direction: None,
            game_over: false,
            grid: grid,
            snake: snake,
            food: food,
            score: 0,
        };
        let mut settings = EventSettings::new();
        settings.set_ups(10);
        settings.set_max_fps(30);
        let mut events = Events::new(settings);
        while let Some(e) = events.next(&mut window) {
            match e {
                Input::Render(_) => {
                    game.on_render(&e);
                },
                Input::Press(_) => {
                    game.on_press(&e);
                },
                Input::Update(_) => {
                    game.on_update();
                },
                _ => {},
            }
        }
    }
}
