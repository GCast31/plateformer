use game2d::{game::{common::{Position2d, Velocity2d, Dimension2d, DeltaTime, Point2d}, inputs::Inputs}, graphics::{graphics::{Draw, Graphics, DrawMode}, color::Color}, inputs::keyboard::Keys};

const PLAYER_ACCEL: f32 = 150.;
const PLAYER_FRICTION: f32 = 150.;
const PLAYER_MAX_SPEED: f32 = 150.; 
const PLAYER_JUMP_VELOCITY: f32 = -280.;

pub struct Player {
    pub position: Position2d,
    pub velocity: Velocity2d,
    pub dimension: Dimension2d,
    pub standing: bool,
    pub jump_ready: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            position: Position2d { x: 0., y: 0. },
            velocity: Velocity2d { vx: 0., vy: 0. },
            dimension: Dimension2d { h: 32, w: 32 },
            standing: false,
            jump_ready: true,
        }
    }
}

impl Draw for Player {
    fn draw(&mut self, graphics: &mut Graphics) {
        graphics.rectangle(
            DrawMode::Fill, 
            self.position.x, 
            self.position.y, 
            self.dimension.w, 
            self.dimension.h, 
            Some(Color::WHITE)
        );
    }
}

impl Player {
    pub fn update(&mut self, inputs: &Inputs, dt: &DeltaTime) {

        // === Friction (effet de glissage)
        if self.velocity.vx > 0. {
            self.velocity.vx -= PLAYER_FRICTION * dt;
            if self.velocity.vx < 0. {
                self.velocity.vx = 0.
            }
        }
        if self.velocity.vx < 0. {
            self.velocity.vx += PLAYER_FRICTION * dt;
            if self.velocity.vx > 0. {
                self.velocity.vx = 0.
            }
        }

        // === KEYBOARD
        // Left
        if inputs.keyboard.is_down(&Keys::Left) {
            self.velocity.vx -= PLAYER_ACCEL * dt;
            if self.velocity.vx < -PLAYER_MAX_SPEED {
                self.velocity.vx = -PLAYER_MAX_SPEED; 
            }
        }
        // Right
        if inputs.keyboard.is_down(&Keys::Right) {
            self.velocity.vx += PLAYER_ACCEL * dt;
            if self.velocity.vx > PLAYER_MAX_SPEED {
                self.velocity.vx = PLAYER_MAX_SPEED; 
            }
        }
        // Up = JUMP
        if inputs.keyboard.is_down(&Keys::Up) {
            if self.standing && self.jump_ready {
                self.velocity.vy = PLAYER_JUMP_VELOCITY;
                self.standing = false;
                self.jump_ready = false;
            }
        }
        else if self.jump_ready == false {
            self.jump_ready = true;
        }

        // === MOVE
        self.position = self.position + self.velocity;
    }
}
