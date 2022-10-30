use game2d::{game::{common::{Position2d, Velocity2d, Size2d, DeltaTime, Sizable, Positionable, Movable, WithPosition, WithSize, Standing, Transformation}, inputs::Inputs, game::Updatable, sprites::SpriteTrait}, graphics::{graphics::{Graphics, DrawMode, Drawable}, color::Color, images::{ImageInformations, Image}, self}, inputs::keyboard::Keys, animations::{animation::{Animation, self}, manager::AnimationsManager}};

use crate::{SpriteCommonPlaterformerTrait, level::MAP_TILE_SIZE};

const PLAYER_ACCEL: f32 = 150.;
const PLAYER_FRICTION: f32 = 150.;
const PLAYER_MAX_SPEED: f32 = 150.; 
const PLAYER_JUMP_VELOCITY: f32 = -400.;

pub trait PlayerTrait {}

pub struct Player {
    position: Position2d,
    velocity: Velocity2d,
    size: Size2d,
    standing: bool,
    jump_ready: bool,
    animations: AnimationsManager,
}

impl SpriteTrait for Player {}

impl SpriteCommonPlaterformerTrait for Player {}

impl Drawable for Player {
    fn draw(&mut self, graphics: &mut Graphics) {
        let image = self.animations.run_current();
        if let Some(image) = image {
          let scalex = (self.size.w / image.get_width()) as Transformation;
          let scaley = (self.size.h / image.get_height()) as Transformation;
          graphics.draw_full(image.as_ref(), self.position.x, self.position.y, 0., scalex, scaley, 0., 0.);
        }
        // graphics.rectangle(
        //     DrawMode::Fill, 
        //     self.position.x, 
        //     self.position.y, 
        //     self.size.w, 
        //     self.size.h, 
        //     Some(Color::WHITE)
        // );
    }
}

impl Player {
    pub fn new(graphics: &mut Graphics) -> Self {

        let mut animations = AnimationsManager::new();

        let mut animation = Animation::new();
        animation.set_timer(100);
        let image = graphics.new_image("images/player/idle1.png").unwrap();
        animation.add(Box::new(image));
        let image = graphics.new_image("images/player/idle2.png").unwrap();
        animation.add(Box::new(image));
        let image = graphics.new_image("images/player/idle3.png").unwrap();
        animation.add(Box::new(image));
        animations.add("idle".to_owned(), animation).unwrap();

        let mut animation = Animation::new();
        animation.set_timer(100);
        let image = graphics.new_image("images/player/run1.png").unwrap();
        animation.add(Box::new(image)); 
        let image = graphics.new_image("images/player/run2.png").unwrap();
        animation.add(Box::new(image));
        let image = graphics.new_image("images/player/run3.png").unwrap();
        animation.add(Box::new(image));
        let image = graphics.new_image("images/player/run4.png").unwrap();
        animation.add(Box::new(image));
        let image = graphics.new_image("images/player/run5.png").unwrap();
        animation.add(Box::new(image));
        let image = graphics.new_image("images/player/run6.png").unwrap();
        animation.add(Box::new(image));
        let image = graphics.new_image("images/player/run7.png").unwrap();
        animation.add(Box::new(image));
        let image = graphics.new_image("images/player/run8.png").unwrap();
        animation.add(Box::new(image));
        let image = graphics.new_image("images/player/run9.png").unwrap();
        animation.add(Box::new(image));
        let image = graphics.new_image("images/player/run10.png").unwrap();
        animation.add(Box::new(image));
        animations.add("run".to_owned(), animation).unwrap();

        let mut animation = Animation::new();
        animation.set_timer(100);
        let image = graphics.new_image("images/player/fall.png").unwrap();
        animation.add(Box::new(image)); 
        animations.add("fall".to_owned(), animation).unwrap();

        let mut animation = Animation::new();
        animation.set_timer(100);
        let image = graphics.new_image("images/player/fall.png").unwrap();
        animation.add(Box::new(image)); 
        animations.add("jump".to_owned(), animation).unwrap();

        animations.set_current("idle".to_owned()).unwrap();

        Self {
            position: Position2d { x: 0., y: 0. },
            velocity: Velocity2d { vx: 0., vy: 0. },
            size: Size2d { h: MAP_TILE_SIZE as u32, w: MAP_TILE_SIZE as u32 },
            standing: true,
            jump_ready: true,
            animations: animations,
        }
    }
}

impl Positionable for Player {
    fn set_position(&mut self, position: Position2d) {
        self.position = position;
    }
    fn set_x(&mut self, x: game2d::game::common::Position) {
        self.position.x = x;
    }
    fn set_y(&mut self, y: game2d::game::common::Position) {
        self.position.y = y;
    }
}

impl Movable for Player {
    fn get_velocity(&self) -> &Velocity2d {
        &self.velocity
    }

    fn get_mut_velocity(&mut self) -> &mut Velocity2d {
        &mut self.velocity
    }

    fn set_velocity(&mut self, velocity: Velocity2d) {
        self.velocity = velocity;
    }
    fn set_vx(&mut self, vx: game2d::game::common::Velocity) {
        self.velocity.vx = vx;
    }
    fn set_vy(&mut self, vy: game2d::game::common::Velocity) {
        self.velocity.vy = vy;
    }
}

impl Sizable for Player {
    fn set_size(&mut self, size: Size2d) {
        self.size = size
    }
}

impl Standing for Player {
    fn get_standing(&self) -> bool {
      self.standing
    }
    fn set_standing(&mut self, standing: bool) {
      self.standing = standing;
    }
}

impl Updatable for Player {
    
    fn update(&mut self, _graphics: &mut Graphics, inputs: &mut Inputs, dt: &DeltaTime) {
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
            self.animations.set_current("run".to_owned()).unwrap();
        }
        // Right
        if inputs.keyboard.is_down(&Keys::Right) {
            self.velocity.vx += PLAYER_ACCEL * dt;
            if self.velocity.vx > PLAYER_MAX_SPEED {
                self.velocity.vx = PLAYER_MAX_SPEED; 
            }
            self.animations.set_current("run".to_owned()).unwrap();
        }
        // Up = JUMP
        if inputs.keyboard.is_down(&Keys::Up) {
            if self.standing && self.jump_ready {
                self.velocity.vy = PLAYER_JUMP_VELOCITY * dt;
                self.standing = false;
                self.jump_ready = false;  
                self.animations.set_current("jump".to_owned()).unwrap();
            }
        }
        else if self.jump_ready == false {
            self.jump_ready = true;
        }

        // === MOVE
        self.position = self.position + self.velocity;
    }
}

impl WithPosition for Player {
    fn get_position(&self) -> &Position2d {
        &self.position
    }
}

impl WithSize for Player {
    fn get_size(&self) -> &Size2d {
        &self.size
    }
}