use std::collections::HashMap;

use game2d::graphics::graphics::{Draw, Drawable};
use game2d::game::common::{Dimension, Position};

pub type MapLevel = Vec<Vec<char>>;

pub struct MapElement {
    pub name: String,
    pub filename: String,
    pub solid: bool,
}

pub struct Map {
    elements: HashMap<char, MapElement>,
    level: MapLevel,
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
    }

    /*
     * get_tile_at()
     * 
     * @Brief: Get tile at pixel
     */
    pub fn get_tile_at(&mut self, x: Position, y: Position) -> Option<&MapElement> {
        let mut map_element = Option::None;
        let col = (x / 32.).floor() as isize;
        let lig = (y / 32.).floor() as isize;
        if col >= 0 && lig >= 0 && lig <= self.level.len() as isize {
            if let Some(at_lig) = self.level.get(lig as usize) {
                if let Some(id_element) = at_lig.get(col as usize) {
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
            vec!['1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1', '1', '1', '1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1'],
            vec!['1', '0', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '0', '0', '1'],
            vec!['1', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '0', '1'],
            vec!['1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1', '1'],
        ];
    
        level
}
