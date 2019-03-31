extern crate ggez;
extern crate rand;
use ggez::event::{self, MouseButton};
use ggez::graphics::{self, Image};
use ggez::mint::{Point2, Vector2};
use ggez::*;
use std::path;

struct WindowState {
    width: f32,
    height: f32,
}

struct Velocity {
    x: f32,
    y: f32,
    max_velocity: f32,
}

impl Velocity {
    fn new(vx: f32, vy: f32, max: f32) -> Velocity {
        return Velocity {
            x: vx,
            y: vy,
            max_velocity: max,
        };
    }
}

struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    time: f32,
    time_passed: f32,
}

static window: WindowState = WindowState {
    width: 640.0,
    height: 480.0,
};

struct MainState {
    football: Image,
    x: f32,
    y: f32,
    velocity: Velocity,
    mouse_down: bool,
}

struct Lerp {
    start: f32,
    end: f32,
    step: f32,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            football: Image::new(_ctx, "/football.png").unwrap(),
            x: 0.0,
            y: window.height as f32 / 2.0,
            mouse_down: false,
            velocity: Velocity::new(0.0, 0.0, 10.0),
        };
        Ok(s)
    }
}

fn world_to_screen_coords(
    screen_width: f32,
    screen_height: f32,
    point: Point2<f32>,
) -> Point2<f32> {
    let x = point.x + screen_width / 2.0;
    let y = screen_height - (point.y + screen_height / 2.0);
    Point2 { x: x, y: y }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let inc = 0.015;
        if self.velocity.y + inc < self.velocity.max_velocity {
            self.velocity.y = self.velocity.y + inc;
        }
        self.y = self.y - self.velocity.y; // 0.15 is the acceleration
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.0, 0.5, 0.0, 1.0].into());
        let pos = world_to_screen_coords(
            window.width,
            window.height,
            Point2 {
                x: self.x,
                y: self.y,
            },
        );

        let draw_params = graphics::DrawParam {
            dest: pos,
            rotation: 0.0,
            scale: Vector2 { x: 0.25, y: 0.25 },
            offset: Point2 { x: 0.5, y: 0.5 },
            ..Default::default()
        };
        graphics::draw(ctx, &self.football, draw_params);
        //        graphics::draw(ctx, &self.football, Point2::new((window.width / 2) as f32, (window.height / 2) as f32), 0.0);
        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mouse_down = true;
        if button == MouseButton::Left {
            self.velocity.y = self.velocity.y - 5.0;
        }
        println!(
            "Mouse button pressed: {:?}, x: {}, y: {}, velocity: {}, {}",
            button, x, y, self.velocity.x, self.velocity.y
        );
    }
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./images");

    let cb = ContextBuilder::new("touchball", "ggez")
        .add_resource_path(resource_dir)
        .window_setup(conf::WindowSetup::default().title("juggleball"))
        .window_mode(
            conf::WindowMode::default()
                .dimensions(window.width, window.height)
                .hidpi(false)
                .maximized(false),
        );

    let (ctx, events_loop) = &mut cb.build()?;
    let game = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, game)
}
