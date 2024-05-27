use std::process::exit;

use rand::Rng;
use tetra::{
    graphics::{self, *},
    input::*,
    math::{self, vec2, Vec2},
    window, Context, ContextBuilder, State,
};

struct GameState {
    player: Player,
    tube: Tube,
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb8(11, 25, 255));
        self.player.texture.draw(ctx, self.player.position);
        self.tube
            .up_texture
            .draw(ctx, Vec2::new(self.tube.center_pos_x, self.tube.up_pos_y));
        self.tube
            .down_texture
            .draw(ctx, Vec2::new(self.tube.center_pos_x, self.tube.down_pos_y));
        Ok(())
    }
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.tube.center_pos_x -= 10.;
        if self.tube.center_pos_x <= -100. {
            self.tube.up_pos_y = rand::thread_rng().gen_range(-100..0) as f32;
            self.tube.down_pos_y = rand::thread_rng().gen_range(500..700) as f32;
            self.tube.center_pos_x = 850.;
        }
        if is_key_pressed(ctx, Key::Space) {
            self.player.velocity.y = -20.;
        }
        self.player.velocity.y += 1.;
        self.tube.up_collider.x = self.tube.center_pos_x;
        self.tube.down_collider.x = self.tube.center_pos_x;
        self.tube.up_collider.y = self.tube.up_pos_y;
        self.tube.down_collider.y = self.tube.down_pos_y;
        self.player.collider.y = self.player.position.y;

        self.player.position += self.player.velocity;
        if self.player.collider.check_collision(&self.tube.up_collider)
            || self
                .player
                .collider
                .check_collision(&self.tube.down_collider)
        {
            exit(0);
        }
        Ok(())
    }
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player_pos = Vec2::new(200., 200.);
        let player_vel = Vec2::new(0., 0.);
        let texture: Texture = Texture::new(ctx, "./assets/tetra-left.png")?;
        let rect = Rect {
            x: player_pos.x,
            y: player_pos.y,
            width: 120.,
            height: 55.,
        };
        let player = Player {
            position: player_pos,
            velocity: player_vel,
            texture: texture,
            collider: rect,
        };
        let tube_pos_x = 850.;
        let up_texture = Texture::new(ctx, "./assets/tube_up.png")?;
        let down_texture = Texture::new(ctx, "./assets/tube_down.png")?;
        let up_tube_pos = rand::thread_rng().gen_range(-100..0) as f32;
        let down_tube_pos = rand::thread_rng().gen_range(500..700) as f32;

        let up_col = Rect {
            x: tube_pos_x,
            y: up_tube_pos,
            width: 120.,
            height: 192.,
        };
        let down_col = Rect {
            x: tube_pos_x,
            y: down_tube_pos,
            width: 120.,
            height: 192.,
        };
        let tube = Tube {
            center_pos_x: tube_pos_x,
            up_texture: down_texture,
            down_texture: up_texture,
            up_pos_y: up_tube_pos,
            down_pos_y: down_tube_pos,
            up_collider: up_col,
            down_collider: down_col,
        };

        Ok(GameState { player, tube })
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Game", 800, 700)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

struct Player {
    position: Vec2<f32>,
    velocity: Vec2<f32>,
    texture: Texture,
    collider: Rect,
}

impl Player {
    fn new(ctx: &mut Context) -> tetra::Result<Player> {
        let texture: Texture = Texture::new(ctx, "./assets/tetra-left.png")?;
        Ok(Player {
            position: Vec2::new(0., 0.),
            velocity: Vec2::new(0.0, 0.),
            texture: texture,
            collider: Rect {
                x: 0.,
                y: 0.,
                width: 120.,
                height: 55.,
            },
        })
    }
}

struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Rect {
    fn check_collision(&self, other: &Rect) -> bool {
        if self.x + self.width < other.x || self.x > other.x + other.width {
            return false;
        }
        if self.y + self.height < other.y || self.y > other.y + other.height {
            return false;
        }

        true
    }
}

struct Tube {
    up_texture: Texture,
    down_texture: Texture,
    center_pos_x: f32,
    up_pos_y: f32,
    down_pos_y: f32,
    up_collider: Rect,
    down_collider: Rect,
}
