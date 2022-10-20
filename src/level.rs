use std::collections::HashMap;

use game2d::graphics::graphics::{Draw, Drawable};
use game2d::game::common::{Dimension, Position, Position2d};

pub type MapLevel = Vec<Vec<char>>;

pub const MAP_TILE_SIZE: f32 = 32.;

#[derive(Default)]
pub struct MapElement {
    pub name: String,
    pub filename: String,
    pub solid: bool,
}


#[derive(Default)]
pub struct MapCoord {
    pub lig: isize,
    pub col: isize,
}

impl From<Position2d> for MapCoord {
    fn from(position: Position2d) -> Self {
        let col = (position.x / MAP_TILE_SIZE).floor() as isize;
        let lig = (position.y / MAP_TILE_SIZE).floor() as isize;
        
        Self { lig, col }
    }
}

impl MapCoord {
    pub fn to_Position2d(&self) -> Position2d {
        Position2d{
            x: (self.col as f32 - 1_f32) * MAP_TILE_SIZE,
            y: (self.lig as f32 - 1_f32) * MAP_TILE_SIZE,
        }
    }
}

pub struct Map {
    elements: HashMap<char, MapElement>,
    level: MapLevel,
    pub player_start: Option<Position2d>,
}

impl Default for Map {
    fn default() -> Self {
        Self { 
            elements: HashMap::new(), 
            level: MapLevel::default(), 
            player_start: None,
        }
    }
}


impl Map {
    /*
     * new()
     * 
     * @brief : Create a new MAP
     */
    pub fn new() -> Self {

        // Create elements of map
        let mut elements = HashMap::new();

        // #### WALL1
        elements.insert('1', 
            MapElement {
                name: "Wall1".to_string(),
                filename: "images/tile1.png".to_string(),
                solid: true,
            }
        );

        Self { 
            elements,
            level: Vec::new(), 
            ..Default::default()
        }
    }

    /*
     * load_level()
     * 
     * @brief: Load a new level
     */
    pub fn load_level(&mut self, level: i32) {
        match level {
           1 => { self.level = level_1(); }
           _ => { self.level = Vec::new(); }
        };
        let player_coords = MapCoord{lig: 14, col: 2};
        self.player_start = Some(player_coords.to_Position2d());
    }

    /*
     * get_tile_at()
     * 
     * @Brief: Get tile at pixel
     */
    pub fn get_tile_at(&mut self, x: Position, y: Position) -> Option<&MapElement> {
        let mut map_element = Option::None;
        let map_coords: MapCoord = MapCoord::from(Position2d{x, y});
        if map_coords.col >= 0 && map_coords.lig >= 0 && map_coords.lig <= self.level.len() as isize {
            if let Some(at_lig) = self.level.get(map_coords.lig as usize) {
                if let Some(id_element) = at_lig.get(map_coords.col as usize) {
                   map_element = self.elements.get(id_element);
                }
            }
        }

        map_element
    }

}

impl Draw for Map {
    fn draw(&mut self, graphics: &mut game2d::graphics::graphics::Graphics) {

        for (pos_l, l) in self.level.iter().enumerate()
        {
            for (pos_c, c) in l.iter().enumerate() {
                let mut image = Err(format!("Informations {} not found", c));
                
                // Search information of element
                if let Some(element) = self.elements.get(c) {
                    image = graphics.new_image(&element.filename);
                } 
                
                // Draw image
                if let Ok(image) = image {
                    graphics.draw_full(
                        &image, 
                        (pos_c as Dimension * image.get_width()) as Position, 
                        (pos_l as Dimension * image.get_height()) as Position, 
                        0., 1., 1., 0., 0. 
                    );
                }
            }
        }        
    }
}

// ################################################################################################################
// #                                             L E V E L                                                        #
// ################################################################################################################

fn level_1() -> MapLevel {
    let level = 
        vec![
            vec!['1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1'],
            vec!['1', '0', '0', '0', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '0', '0', '0', '1'],
            vec!['1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1', '1'],
            vec!['1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1'],
            vec!['1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1', '1', '1'],
            vec!['1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1'],
            vec!['1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1', '1', '1', '1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1'],
            vec!['1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1'],
            vec!['1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1', '1', '1', '1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1'],
            vec!['1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1'],
            vec!['1', '1', '1', '1', '1', '1', '1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1'],
            vec!['1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1'],
            vec!['1', '1', '0', '0', '0', '0', '0', '0', '0', '0', '1', '1', '1', '1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1'],
            vec!['1', '0', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1'],
            vec!['1', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '0', '1'],
            vec!['1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
        ];
    
        level
}
