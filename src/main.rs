/*
  ██████  ██       █████  ████████ ███████ ███████  ██████  ██████  ███    ███ ███████ ██████  
  ██   ██ ██      ██   ██    ██    ██      ██      ██    ██ ██   ██ ████  ████ ██      ██   ██ 
  ██████  ██      ███████    ██    █████   █████   ██    ██ ██████  ██ ████ ██ █████   ██████  
  ██      ██      ██   ██    ██    ██      ██      ██    ██ ██   ██ ██  ██  ██ ██      ██   ██ 
  ██      ███████ ██   ██    ██    ███████ ██       ██████  ██   ██ ██      ██ ███████ ██   ██
    
    @Author : GCast31
*/

pub mod level;
pub mod player;

use std::any::TypeId;

use game2d::game::common::{GAME_FONT_DEFAULT_, GAME_FONT_DEFAULT_SIZE, DeltaTime, Position2d, Position, Positionable, WithPosition, WithSize, Movable, Standing, Scale2d};
use game2d::game::game::*;
use game2d::game::inputs::Inputs;
use game2d::game::sprites::Sprites;
use game2d::graphics::color::Color;
use game2d::graphics::fonts::FontsManager;
use game2d::graphics::graphics::{Graphics, Drawable};
use game2d::inputs::keyboard::Keys;
use level::{Map, MapCoord};
use player::Player;


// ################################################################################################################
// #                                      C O N S T R A N T E S  FOR  G A M E                                     #
// ################################################################################################################
pub const GAME_WINDOW_HEIGHT: u32 = 600;
pub const GAME_WINDOW_WIDTH: u32 = 800;
pub const GAME_SCALE: Scale2d = Scale2d {sx: 1. , sy: 1.};

const SPRITE_FALLING: f32 = 20.;

// ################################################################################################################
// #                                        S T R U C T U R E    G A M E                                          #
// ################################################################################################################

pub struct Plateformer {
    actual_level: i32,
    map: Map,
    list_sprites: Sprites,
}

impl Default for Plateformer {
    fn default() -> Self {
        Plateformer { actual_level: 0, map: Map::new(), list_sprites: Sprites::new() }
    }
}

// ################################################################################################################
// #                                        S P E C I A L   T R A I T                                             #
// ################################################################################################################
pub trait SpriteCommonPlaterformerTrait: WithPosition + WithSize + Movable + Standing {} 

// ################################################################################################################
// #                                                   M A I N                                                    #
// ################################################################################################################
fn main() {

    let mut graphics = Graphics::new(
        "Plaformer", 
        GAME_WINDOW_WIDTH, 
        GAME_WINDOW_HEIGHT, 
        false
    ).unwrap();

    // Fonts
    let mut font_context = Graphics::create_fonts_context();
    let mut fonts_manager: FontsManager = FontsManager::new(graphics.get_fonts_creator());
    let font_detail = fonts_manager.load_font(&mut font_context, GAME_FONT_DEFAULT_.to_string(), GAME_FONT_DEFAULT_SIZE).unwrap();
    graphics.set_font(font_detail);
    graphics.set_scale(GAME_SCALE);

    // Game
    Game::new(graphics)
        .set_params(Plateformer::default())
        .set_max_fps(Some(144.))
        .set_callback_draw(draw)
        .set_callback_load(load)
        .set_callback_key_pressed(keypressed)
        .set_callback_update(update)
        .set_callback_quit(quit)
        .run(&mut Some(fonts_manager));

}


// ################################################################################################################
// #                                                    L O A D                                                   #
// ################################################################################################################
#[allow(unused_variables)]
pub fn load(graphics: &mut Graphics, game: &mut Option<Plateformer>) {
    // Set background color
    graphics.set_background_color(Color::BLACK);


    if let Some(game) = game {
        // Load initial level
        game.actual_level = 1;
        game.map.load_level(game.actual_level);

        // Add Player
        let player_position: Position2d;
        if let Some(player_start) = game.map.player_start {
            player_position = player_start;  
        } else {
            player_position = Position2d {x: 0., y: 0.};
        }

        let mut player = Player::new(graphics);
        player.set_position(player_position);
        game.list_sprites.add(player);
    }
}

// ################################################################################################################
// #                                                   U P D A T E                                                #
// ################################################################################################################ 
#[allow(unused_variables)]
pub fn update(graphics: &mut Graphics, game: &mut Option<Plateformer>, inputs: &mut Inputs, dt: DeltaTime) {
    if let Some(game) = game {

        // Sprites
        let sprites = game.list_sprites.get_all_mut();

        for (typeid, list) in sprites.iter_mut() {
            for sprite in list.iter_mut() {
               if let Some(player) = sprite.downcast_mut::<Player>() {
                  player.update(graphics, inputs, &dt);
                  update_sprite(typeid, player, &game.map, &dt)
               }
            }
        }
    }
}

fn update_sprite<T: SpriteCommonPlaterformerTrait>(_typeid: &TypeId, sprite: &mut T, map: &Map, dt: &DeltaTime) {
    let _sprite_position = sprite.get_position().clone();

    // Collide detection
    let mut collide = false;

    let mut velocity = *sprite.get_mut_velocity();

    let mut modify_position = false;

    // -- Right 
    if !collide && velocity.vx > 0. {
        collide = map.collide(level::MapElementCollideType::Right, sprite);
    }

    // -- Left
    if !collide && velocity.vx < 0. {
        collide = map.collide(level::MapElementCollideType::Left, sprite);
    }

    // -- Stop !
    if collide {
        velocity.vx = 0.;
        sprite.set_x(((sprite.get_position().x + (sprite.get_size().w as Position / 2.) / sprite.get_size().w as Position)).floor());
    }
    collide = false;

    // Above
    if !collide && velocity.vy < 0. {
        collide = map.collide(level::MapElementCollideType::Above, sprite);
        if collide {
          velocity.vy = 0.;
          sprite.set_y(((sprite.get_position().y + (sprite.get_size().h as Position / 2.) / sprite.get_size().h as Position)).floor());
          modify_position = true;
        }
    }

    // Below
    if sprite.get_standing() || velocity.vy > 0. {
        collide = map.collide(level::MapElementCollideType::Below, sprite);
        if collide {
            sprite.set_standing(true);
            velocity.vy = 0.;
            sprite.set_y(((sprite.get_position().y + (sprite.get_size().w as Position / 2.) / sprite.get_size().h as Position)).floor());
            modify_position = true;
        }
        else {
            sprite.set_standing(false);
        }
    }
    // Sprite falling
    if sprite.get_standing() == false {
        velocity.vy += SPRITE_FALLING * dt;
    }

    if modify_position {
        //let coord = MapCoord::from(sprite.get_position().clone());
        //println!("{:?}, {:?}", _sprite_position.x, _sprite_position.y);
        //println!("{:?}, {:?}", coord.col, coord.lig);
        //sprite.set_position(MapCoord::to_position2d(coord));
    }

    sprite.set_velocity(velocity);
}


// ################################################################################################################
// #                                               K E Y P R E S S E D                                            #
// ################################################################################################################ 
#[allow(unused_variables)]
pub fn keypressed(graphics: &mut Graphics, game: &mut Option<Plateformer>, key: &Keys) {
    
}

// ################################################################################################################
// #                                                    D R A W                                                   #
// ################################################################################################################ 
#[allow(unused_variables)]
pub fn draw(graphics: &mut Graphics, game: &mut Option<Plateformer>, inputs: &mut Inputs, fonts_manager: &mut Option<FontsManager>) {
   if let Some(game) = game {
        // Draw the map
        game.map.draw(graphics);

        // Debug
        if let Some(fonts_manager) = fonts_manager {
            if let Some(element) = game.map.get_tile_at(inputs.mouse.get_x(), inputs.mouse.get_y()) {
                graphics.print(fonts_manager, element.filename.clone(), 0., GAME_WINDOW_HEIGHT as Position - 20., Option::None);
            
            } else {
                graphics.print(fonts_manager, "Nothing".to_string(), 0., GAME_WINDOW_HEIGHT as Position - 20., Option::None);
            }
        }

        // Draw sprites
        let sprites = game.list_sprites.get_all_mut();

        for (typeid, list) in sprites.iter_mut() {
            for sprite in list.iter_mut() {
                if let Some(player) = sprite.downcast_mut::<Player>() {
                    player.draw(graphics);
                }
            }
        } 
    
    }
}

// ################################################################################################################
// #                                                    Q U I T                                                   #
// ################################################################################################################ 
#[allow(unused_variables)]
pub fn quit(graphics: &mut Graphics, game: &mut Option<Plateformer>) {
    println!("Bye");
}