/*
  ██████  ██       █████  ████████ ███████ ███████  ██████  ██████  ███    ███ ███████ ██████  
  ██   ██ ██      ██   ██    ██    ██      ██      ██    ██ ██   ██ ████  ████ ██      ██   ██ 
  ██████  ██      ███████    ██    █████   █████   ██    ██ ██████  ██ ████ ██ █████   ██████  
  ██      ██      ██   ██    ██    ██      ██      ██    ██ ██   ██ ██  ██  ██ ██      ██   ██ 
  ██      ███████ ██   ██    ██    ███████ ██       ██████  ██   ██ ██      ██ ███████ ██   ██
    
    @Author : GCast31
*/

pub mod level;

use game2d::game::common::{GAME_FONT_DEFAULT_, GAME_FONT_DEFAULT_SIZE, DeltaTime, Position};
use game2d::game::game::*;
use game2d::graphics::color::Color;
use game2d::graphics::fonts::FontsManager;
use game2d::graphics::graphics::{Graphics, Draw, DrawMode};

use game2d::inputs::keyboard::Keyboard;
use game2d::inputs::keyboard::Keys;

use level::Map;

/*****
 * TEST
 *****/
 struct Player {
    x: Position,
    y: Position,
 }

// ################################################################################################################
// #                                      C O N S T R A N T E S  FOR  G A M E                                     #
// ################################################################################################################
pub const GAME_WINDOW_HEIGHT: u32 = 600;
pub const GAME_WINDOW_WIDTH: u32 = 800;


// ################################################################################################################
// #                                        S T R U C T U R E    G A M E                                          #
// ################################################################################################################
pub struct Plateformer {
    actual_level: i32,
    map: Map,
    player: Player,
}

#[allow(dead_code)]
impl Default for Plateformer {
    fn default() -> Self {
        Plateformer {
            actual_level: 0,
            map: Map::new(),
            player : Player { x: 0., y: 0.},
        }
    }
}


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

    // Load initial level
    if let Some(game) = game {
        game.actual_level = 1;
        game.map.load_level(game.actual_level);
    }

}

// ################################################################################################################
// #                                                   U P D A T E                                                #
// ################################################################################################################ 
#[allow(unused_variables)]
pub fn update(graphics: &mut Graphics, game: &mut Option<Plateformer>, keyboard: &mut Keyboard, dt: DeltaTime) {
    if let Some(game) = game {
        if keyboard.is_down(&Keys::Left) {
            game.player.x -= 64. * dt;
        }
        if keyboard.is_down(&Keys::Right) {
            game.player.x += 64. * dt;
        }
        if keyboard.is_down(&Keys::Up) {
            game.player.y -= 64. * dt;
        }
        if keyboard.is_down(&Keys::Down) {
            game.player.y += 64. * dt;
        }
    }
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
pub fn draw(graphics: &mut Graphics, game: &mut Option<Plateformer>, fonts_manager: &mut Option<FontsManager>) {
   if let Some(game) = game {
        // Draw the map
        game.map.draw(graphics);

        // Draw player
        graphics.rectangle(DrawMode::Fill, game.player.x, game.player.y, 32, 32, Some(Color::WHITE));
   
        // Debug
        if let Some(fonts_manager) = fonts_manager {
            if let Some(element) = game.map.get_tile_at(game.player.x, game.player.y) {
                graphics.print(fonts_manager, element.filename.clone(), 0., GAME_WINDOW_HEIGHT as Position - 20., Option::None);
            
            } else {
                graphics.print(fonts_manager, "Nothing".to_string(), 0., GAME_WINDOW_HEIGHT as Position - 20., Option::None);
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