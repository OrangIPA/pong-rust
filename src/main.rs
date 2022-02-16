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
    speed: f64,
    kuadran: (MovementHorizontal, MovementVertical),
}

impl Ball {
    fn new() -> Ball{
        let randdir: i32 = rand::thread_rng().gen_range(0..3);
        let randdir = match randdir{
            0 => (MovementHorizontal::Left, MovementVertical::Up),
            1 => (MovementHorizontal::Left, MovementVertical::Down),
            2 => (MovementHorizontal::Right, MovementVertical::Up),
            3 => (MovementHorizontal::Right, MovementVertical::Down),
            _ => (MovementHorizontal::None, MovementVertical::None),
        };
        Ball {
            pos: (250.0, 150.0),
            dir: rand::thread_rng().gen_range(0.5..(0.5 * PI - 0.5)),
            speed: 8.0,
            kuadran: randdir,
        }
    }

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

    fn update(&mut self, pad: &mut (Paddle, Paddle)){
        let dx = self.speed * self.dir.sin();
        let dy = self.speed * self.dir.cos();

        if self.pos.1 < pad.0.pos + 5.0{
            pad.0.up_slow();
        } else if self.pos.1 > pad.0.pos - 5.0{
            pad.0.down_slow();
        }

        // Top Bounce
        if self.kuadran.1 == MovementVertical::Up {
            if dy < self.top_margin(){
                self.pos.1 -= dy;
            }else {
                self.pos.1 += self.top_margin() - dy;
                self.kuadran.1 = MovementVertical::Down;
            }
        // Bottom Bounce
        }else if self.kuadran.1 == MovementVertical::Down {
            if dy < self.bottom_margin(){
                self.pos.1 += dy;
            }else {
                self.pos.1 -= self.bottom_margin() - dy;
                self.kuadran.1 = MovementVertical::Up;
            }
        }

        // Right Bounce
        if self.kuadran.0 == MovementHorizontal::Right{

            // Check if the Ball can get hit by the Paddle
            if dx < self.right_margin(){

                // If the Ball should not get hit by the Paddle yet, update the position without bouncing
                self.pos.0 += dx;
            }else {

                // Check if the Ball hit the Paddle
                if (self.pos.1 > pad.1.pos - 5.0) && (self.pos.1 < (pad.1.pos + pad.1.width + 5.0)){

                    // If the Ball hit the Paddle, bounce it back
                    self.pos.0 -= self.right_margin() - dx;
                    
                    // Change the Ball direction according to the Ball's location while hitting the Paddle
                    self.dir = ((PI * (self.paddle_pos(&pad.1) - 30.0) / 90.0) - 0.5).abs().sin();
                    self.kuadran.1 = if self.paddle_pos(&pad.1) > 0.0 {
                        MovementVertical::Up
                    } else {
                        MovementVertical::Down
                    };

                    self.kuadran.0 = MovementHorizontal::Left;
                }else{

                    // If the Paddle miss the Ball, reset the Ball to the center
                    self.pos = (250.0, 150.0);
                    let randdir: i32 = rand::thread_rng().gen_range(0..3);
                    let randdir = match randdir{
                        0 => (MovementHorizontal::Left, MovementVertical::Up),
                        1 => (MovementHorizontal::Left, MovementVertical::Down),
                        2 => (MovementHorizontal::Right, MovementVertical::Up),
                        3 => (MovementHorizontal::Right, MovementVertical::Down),
                        _ => (MovementHorizontal::None, MovementVertical::None),
                    };
                    self.kuadran = randdir;
                    self.dir =  rand::thread_rng().gen_range(0.5..(0.5 * PI - 0.5));
                }
            }
        // Left Bounce
        }else if self.kuadran.0 == MovementHorizontal::Left{

            // Check if the Ball can get hit by the Paddle
            if dx < self.left_margin(){

                // If the Ball should not get hit the Paddle yet, update the position without bouncing
                self.pos.0 -= dx;
            }else {

                // Check if the Ball hit the Paddle
                if (self.pos.1 > pad.0.pos - 5.0) && (self.pos.1 < (pad.0.pos + pad.0.width + 5.0)){

                    // If the Ball hit the Paddle, bounce it back
                    self.pos.0 += self.left_margin() - dx;

                    // Change the Ball direction according to the Ball's location while hitting the Paddle
                    self.dir = ((PI * (self.paddle_pos(&pad.0) - 30.0) / 90.0) - 0.5).abs().sin();
                    self.kuadran.1 = if self.paddle_pos(&pad.0) > 0.0 {
                        MovementVertical::Up
                    } else {
                        MovementVertical::Down
                    };
                    self.kuadran.0 = MovementHorizontal::Right;
                }else{

                    // If the Paddle miss the Ball, reset the Ball to the center
                    self.pos = (250.0, 150.0);
                    let randdir: i32 = rand::thread_rng().gen_range(0..3);
                    let randdir = match randdir{
                        0 => (MovementHorizontal::Left, MovementVertical::Up),
                        1 => (MovementHorizontal::Left, MovementVertical::Down),
                        2 => (MovementHorizontal::Right, MovementVertical::Up),
                        3 => (MovementHorizontal::Right, MovementVertical::Down),
                        _ => (MovementHorizontal::None, MovementVertical::None),
                    };
                    self.kuadran = randdir;
                    self.dir =  rand::thread_rng().gen_range(0.5..(0.5 * PI - 0.5));
                }
            }
        }

    }

    fn paddle_pos(&self, pad: &Paddle) -> f64{
        -(self.pos.1 - (pad.pos + (pad.width / 2.0)))
    }

    fn top_margin(&self) -> f64{
        self.pos.1 - 8.0
    }

    fn bottom_margin(&self) -> f64 {
        292.0 - self.pos.1
    }

    fn left_margin(&self) -> f64 {
        self.pos.0 - 16.0
    }

    fn right_margin(&self) -> f64 {
        484.0 - self.pos.0
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
        self.ball.update(&mut self.paddle);
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
    pos: f64,
    width: f64,
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

    fn up_slow(&mut self){
        if self.pos > 0.0{
            self.pos -= 5.0;
        }
    }

    fn down_slow(&mut self){
        if self.pos < (300.0 - self.width){
            self.pos += 5.0;
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

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        paddle: (
            Paddle{
                pos: 0.0,
                width: 50.0,
                player: Player::PlayerOne,
                movement: MovementVertical::None,
            },
            Paddle{
                pos: 0.0,
                width: 50.0,
                player: Player::PlayerTwo,
                movement: MovementVertical::None,
            }
        ),
        ball: Ball::new(),
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
