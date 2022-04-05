use std::{env, path};
use ggez::{event::{self, KeyCode}, Context, GameResult, graphics::{self, Color, DrawMode, DrawParam}, conf::{WindowMode, WindowSetup}, timer};
use ggez::graphics::Transform;
use ggez::input::keyboard;
use ggez::input::mouse::CursorIcon::Text;
use glam::*;

// constants
const WINDOW_SIZE: (f32, f32) = (1000.0, 700.0);
const DESIRED_FPS: u32 = 75; // set this to your screen refresh rate (or higher) otherwise it can feel laggy

// player related
const PADDLE_SIZE: (f32, f32) = (10.0, 100.0);
const PLAYER_SPEED: f32 = 700.0; // y axis

// players
struct Player {
    pos: glam::Vec2,
    points: u16,
}

impl Player {
    fn new(pos: glam::Vec2) -> Self {
        Self {
            pos,
            points: 0,
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [self.pos.x, self.pos.y, PADDLE_SIZE.0, PADDLE_SIZE.1].into(),
            Color::WHITE
        )?;
        graphics::draw(ctx, &rectangle, DrawParam::default())?;

        Ok(())
    }

    fn handle_input(&mut self, ctx: &mut Context, delta: f32, use_arrows: bool) {
        let key_up = if use_arrows { KeyCode::Up } else { KeyCode::W };
        let key_down = if use_arrows { KeyCode::Down } else { KeyCode::S };

        if keyboard::is_key_pressed(ctx, key_up) {
            self.pos.y -= PLAYER_SPEED * delta;
        }
        if keyboard::is_key_pressed(ctx, key_down) {
            self.pos.y += PLAYER_SPEED * delta;
        }

        // check for boundaries
        let screen_h = graphics::drawable_size(ctx).1;
        self.pos.y = self.pos.y.clamp(10.0, screen_h - PADDLE_SIZE.1 - 10.0);
    }
}

// game
struct MainState {
    player: Player,
    opponent: Player,
    font: graphics::Font,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let font = graphics::Font::new(ctx, "/RobotoMono-Bold.ttf")?;

        Ok(Self {
            player: Player::new([25.0, 10.0].into()),
            opponent: Player::new([WINDOW_SIZE.0 - PADDLE_SIZE.0 - 25.0, 10.0].into()),
            font,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let delta = 1.0 / DESIRED_FPS as f32;

            self.player.handle_input(ctx, delta, false);
            self.opponent.handle_input(ctx, delta, true);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        self.player.draw(ctx)?;
        self.opponent.draw(ctx)?;

        // score text
        let text = graphics::Text::new((format!("{} - {}", self.player.points, self.opponent.points), self.font, 48.0));
        let text_x = graphics::drawable_size(ctx).0 / 2.0 - text.width(ctx) / 2.0;
        graphics::draw(ctx, &text, ([text_x, 10.0],))?;

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("ping-pong", "HitoIRL")
        .window_setup(WindowSetup::default().title("Ping Pong"))
        .window_mode(WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1))
        .add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state);
}
