extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod objects;


use graphics::{ clear, rectangle };
use graphics::color::hex;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::{ Events, EventLoop, EventSettings };
use piston::input::{ Button, RenderEvent, PressEvent, Input };
use piston::input::keyboard::Key;
use piston::window::{ Size, Window as PistonWindow, WindowSettings };

use objects::Direction;
// use objects::Grid;


pub struct Game {
    gl: GlGraphics,
    direction: Direction,
    x: f64,
    y: f64,
}


impl Game {
    fn on_press(&mut self, e: &Input) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up if self.direction != Direction::Down => {
                    self.direction = Direction::Up;
                },
                Key::Right if self.direction != Direction::Left => {
                    self.direction = Direction::Right;
                },
                Key::Down if self.direction != Direction::Up => {
                    self.direction = Direction::Down;
                }
                Key::Left if self.direction != Direction::Right => {
                    self.direction = Direction::Left;
                },
                _ => {}
            }
        }
    }

    fn on_update(&mut self) {
        match self.direction {
            Direction::Up => {
                self.y = f64::max(self.y - 10f64, 0f64);
            },
            Direction::Right => {
                self.x = f64::min(self.x + 10f64, 800f64);
            },
            Direction::Down => {
                self.y = f64::min(self.y + 10f64, 800f64);
            },
            Direction::Left => {
                self.x = f64::max(self.x - 10f64, 0f64);
            },
            _ => {}
        }
    }

    fn on_render(&mut self, e: &Input) {
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let args = e.render_args().unwrap();
        let square = rectangle::square(self.x, self.y, 200.0);
        let snake_color = hex("3c53a0");

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);
            rectangle(snake_color, square, c.transform, gl);
        });
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
        let Size { width, height } = window.size();
        let mut game = Game {
            gl: GlGraphics::new(opengl),
            direction: Direction::NoDirection,
            x: (width as f64 / 2f64) - 100f64,
            y: (height as f64 / 2f64) - 100f64
        };
        let mut settings = EventSettings::new();
        settings.set_ups(30);
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
