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

use game2d::game::common::{GAME_FONT_DEFAULT_, GAME_FONT_DEFAULT_SIZE, DeltaTime, Velocity2d, Position2d, Position, Dimension2d};
use game2d::game::game::*;
use game2d::game::inputs::Inputs;
use game2d::graphics::color::Color;
use game2d::graphics::fonts::FontsManager;
use game2d::graphics::graphics::{Graphics, Draw, DrawMode, Drawable};
use game2d::inputs::keyboard::Keys;

use level::Map;
use player::Player;

/*****
 * TEST
 *****/



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
    entities: Entities,
}

impl Default for Plateformer {
    fn default() -> Self {
        Plateformer { actual_level: 0, map: Map::new(), entities: Entities ::default() }
    }
}

#[derive(Default)]
pub struct Entities {
    player: Option<Player>,
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

        let player = Player {
            position: player_position,
            ..Default::default()
        };
        game.entities.player = Some(player);
    }
}

// ################################################################################################################
// #                                                   U P D A T E                                                #
// ################################################################################################################ 
#[allow(unused_variables)]
pub fn update(graphics: &mut Graphics, game: &mut Option<Plateformer>, inputs: &mut Inputs, dt: DeltaTime) {
    if let Some(game) = game {
        // Move player
        if let Some(player) = &mut game.entities.player {
            player.update(&inputs, &dt);
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

        // Draw player
        if let Some(player) = &mut game.entities.player {
            player.draw(graphics);
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