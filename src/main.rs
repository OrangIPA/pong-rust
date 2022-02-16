use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

use rand::Rng;
use std::f64::consts::PI;

#[derive(Clone, PartialEq)]
enum MovementVertical {
    Up, Down, None
}

#[derive(Clone, PartialEq)]
enum MovementHorizontal {
    Left, Right, None,
}

enum Player {
    PlayerOne, PlayerTwo
}


struct Ball {
    pos: (f64, f64),
    dir: f64,
    is_alive: bool,
    speed: f64,
    kuadran: (MovementHorizontal, MovementVertical),
}

impl Ball {
    fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs){
        use graphics;
        let WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let circle = graphics::ellipse::circle(
            self.pos.0 as f64,
            self.pos.1 as f64,
            8_f64,
        );
        gl.draw(arg.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::ellipse(WHITE, circle, transform, gl);
        });
    }

    fn update(&mut self){
        let dx = self.speed * self.dir.sin();
        let dy = self.speed * self.dir.cos();
        if self.kuadran.1 == MovementVertical::Up {
            if dy < self.top_margin(){
                self.pos.1 -= dy;
            }else {
                self.pos.1 += self.top_margin() - dy;
                self.kuadran.1 = MovementVertical::Down;
            }
        }else if self.kuadran.1 == MovementVertical::Down {
            if dy < self.bottom_margin(){
                self.pos.1 += dy;
            }else {
                self.pos.1 -= self.bottom_margin() - dy;
                self.kuadran.1 = MovementVertical::Up;
            }
        }

        if self.kuadran.0 == MovementHorizontal::Right{
            if dx < self.right_margin(){
                self.pos.0 += dx;
            }else {
                self.pos.0 -= self.right_margin() - dx;
                self.kuadran.0 = MovementHorizontal::Left;
            }
        }else if self.kuadran.0 == MovementHorizontal::Left{
            if dx < self.left_margin(){
                self.pos.0 -= dx;
            }else {
                self.pos.0 += self.left_margin() - dx;
                self.kuadran.0 = MovementHorizontal::Right;
            }
        }

    }

    fn top_margin(&self) -> f64{
        self.pos.1 - 8.0
    }

    fn bottom_margin(&self) -> f64 {
        292.0 - self.pos.1
    }

    fn left_margin(&self) -> f64 {
        self.pos.0 - 8.0
    }

    fn right_margin(&self) -> f64 {
        492.0 - self.pos.0
    }
}

struct Game {
    gl: GlGraphics,
    paddle: (Paddle, Paddle),
    ball: Ball,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs){
        use graphics;

        let BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(BLACK, gl);
        });

        self.paddle.0.render(&mut self.gl, arg);
        self.paddle.1.render(&mut self.gl, arg);
        self.ball.render(&mut self.gl, arg)
    }

    fn update(&mut self){
        self.paddle.0.update();
        self.paddle.1.update();
        self.ball.update();
    }

    fn pressed(&mut self, btn: &Button){
        self.paddle.1.movement = match btn {
            &Button::Keyboard(Key::Up) => MovementVertical::Up,
            &Button::Keyboard(Key::Down) => MovementVertical::Down,
            _ => MovementVertical::None,
        }
    }

    fn released(&mut self, btn: &Button) {
        self.paddle.1.movement = MovementVertical::None;
    }
}

struct Paddle {
    pos: f32,
    width: f32,
    player: Player,
    movement: MovementVertical,
}

impl Paddle {
    fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs){
        use graphics;

        let xpos = match self.player{
            Player::PlayerOne => 0.0,
            Player::PlayerTwo => 492.0,
        };

        let WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let rectangle = graphics::rectangle::rectangle_by_corners(
            xpos as f64,
            self.pos as f64,
            (xpos + 8.0) as f64,
            (self.pos + self.width) as f64,
        );
        gl.draw(arg.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(WHITE, rectangle, transform, gl);
        });
    }

    fn update(&mut self){
        match self.movement {
            MovementVertical::None => (),
            MovementVertical::Up => { self.up() },
            MovementVertical::Down => { self.down() },
        }
    }

    fn up(&mut self){
        if self.pos > 0.0{
            self.pos -= 10.0;
        }
    }

    fn down(&mut self){
        if self.pos < (300.0 - self.width){
            self.pos += 10.0;
        } 
    }
}

fn main() {
    let opengl = OpenGL::V3_3;
    let mut window: Window = WindowSettings::new(
        "Pong",
        (500, 300)
    ).graphics_api(opengl)
    .exit_on_esc(false)
    .build()
    .unwrap();

    let randdir: i32 = rand::thread_rng().gen_range(0..3);
    let randdir = match randdir{
        0 => (MovementHorizontal::Left, MovementVertical::Up),
        1 => (MovementHorizontal::Left, MovementVertical::Down),
        2 => (MovementHorizontal::Right, MovementVertical::Up),
        3 => (MovementHorizontal::Right, MovementVertical::Down),
        _ => (MovementHorizontal::None, MovementVertical::None),
    };

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        paddle: (
            Paddle{
                pos: 0.0,
                width: 40.0,
                player: Player::PlayerOne,
                movement: MovementVertical::None,
            },
            Paddle{
                pos: 0.0,
                width: 40.0,
                player: Player::PlayerTwo,
                movement: MovementVertical::None,
            }
        ),
        ball: Ball {
            pos: (250.0, 150.0),
            dir: rand::thread_rng().gen_range(0.0..(0.5 * PI)),
            is_alive: true,
            speed: 10.0,
            kuadran: randdir,
        }
    };
    
    let mut events = Events::new(EventSettings::new()).ups(30);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args(){
            game.render(&args);
        }

        if let Some(u) = e.update_args(){
            game.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press{
                game.pressed(&k.button);
            }
            if k.state ==ButtonState::Release{
                game.released(&k.button);
            }
        }
    }
}
