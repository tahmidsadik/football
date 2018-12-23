extern crate ggez;
extern crate rand;
use ggez::event::{self, MouseButton};
use ggez::graphics::{self, Image, Point2};
use ggez::*;

struct WindowState {
    width: u32,
    height: u32,
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
    width: 1024,
    height: 768,
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
            football: Image::new(_ctx, "/images/football.jpg").unwrap(),
            x: 0.0,
            y: window.height as f32 / 2.0,
            mouse_down: false,
            velocity: Velocity::new(0.0, 0.0, 10.0),
        };
        Ok(s)
    }
}

fn world_to_screen_coords(screen_width: u32, screen_height: u32, point: Point2) -> Point2 {
    let width = screen_width as f32;
    let height = screen_height as f32;
    let x = point.x + width / 2.0;
    let y = height - (point.y + height / 2.0);
    Point2::new(x, y)
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let inc = 0.015;
        if (self.velocity.y + inc < self.velocity.max_velocity) {
            self.velocity.y = self.velocity.y + inc;
        }
        self.y = self.y - self.velocity.y; // 0.15 is the acceleration
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let pos = world_to_screen_coords(window.width, window.height, Point2::new(self.x, self.y));

        let drawParams = graphics::DrawParam {
            dest: pos,
            rotation: 0.0,
            scale: Point2::new(0.25, 0.25),
            offset: Point2::new(0.5, 0.5),
            ..Default::default()
        };
        graphics::draw_ex(ctx, &self.football, drawParams);
        //        graphics::draw(ctx, &self.football, Point2::new((window.width / 2) as f32, (window.height / 2) as f32), 0.0);
        graphics::present(ctx);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        self.mouse_down = true;
        if (button == MouseButton::Left) {
            self.velocity.y = self.velocity.y - 5.0;
        }
        println!(
            "Mouse button pressed: {:?}, x: {}, y: {}, velocity: {}, {}",
            button, x, y, self.velocity.x, self.velocity.y
        );
    }
}

pub fn main() {
    let c = conf::Conf {
        window_mode: conf::WindowMode {
            width: window.width,
            height: window.height,
            borderless: false,
            fullscreen_type: conf::FullscreenType::Off,
            vsync: true,
            min_width: 0,
            max_width: 0,
            min_height: 0,
            max_height: 0,
        },
        window_setup: conf::WindowSetup::default(),
        backend: conf::Backend::OpenGL { major: 3, minor: 2 },
    };

    let mut ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();

    event::run(ctx, state).unwrap();
}
