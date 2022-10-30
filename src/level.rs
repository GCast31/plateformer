use std::collections::HashMap;

use game2d::{game::common::{Size, Position, Position2d, WithPosition, WithSize, Transformation}, graphics::{graphics::Drawable, images::ImageInformations}};

pub type MapLevel = Vec<Vec<char>>;

pub const MAP_TILE_SIZE: f32 = 32.;

pub enum MapElementCollideType {
    Right,
    Left,
    Below,
    Above,
}

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
    pub fn to_position2d(coord: MapCoord) -> Position2d {
        Position2d{
            x: (coord.col as f32 - 1_f32) * MAP_TILE_SIZE,
            y: (coord.lig as f32 - 1_f32) * MAP_TILE_SIZE,
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
        self.player_start = Some(MapCoord::to_position2d(MapCoord{lig: 14, col: 2}));
    }

    /*
     * get_tile_at()
     * 
     * @Brief: Get tile at pixel
     */
    pub fn get_tile_at(&self, x: Position, y: Position) -> Option<&MapElement> {
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

    /*
     * collide()
     * 
     * @Brief: Check if entity collide with mapelement
     */
    pub fn collide<T: WithPosition + WithSize + ?Sized>(&self, type_collide: MapElementCollideType, entity: &T) -> bool {

        let entity_size = entity.get_size();
        let entity_position = entity.get_position();

        let height: Size = if entity_size.h > MAP_TILE_SIZE as Size { MAP_TILE_SIZE as Size } else { entity_size.h };
        let width: Size = if entity_size.w > MAP_TILE_SIZE as Size { MAP_TILE_SIZE as Size } else { entity_size.w };


        let (position1, position2) =
            match type_collide {
                MapElementCollideType::Above => {
                    (
                        Position2d {x: entity_position.x + 1., y: entity_position.y - 1. },
                        Position2d {x: entity_position.x + width as Position - 2.,y: entity_position.y - 1. }
                    )
                },
                MapElementCollideType::Below => {
                    (
                        Position2d {x: entity_position.x + 1., y: entity_position.y + width as Position },
                        Position2d {x: entity_position.x + width as Position - 2.,y: entity_position.y + width as Position }
                    )
                },
                MapElementCollideType::Left => {
                    (
                        Position2d {x: entity_position.x - 1., y: entity_position.y + 3. },
                        Position2d {x: entity_position.x - 1. ,y: entity_position.y + height as Position - 2.}
                    )
                },
                MapElementCollideType::Right => {
                    (
                        Position2d {x: entity_position.x + width as Position, y: entity_position.y + 3. },
                        Position2d {x: entity_position.x + width as Position ,y: entity_position.y + height as Position - 2.}
                    )
                }
            };

        let id1 = self.get_tile_at(position1.x, position1.y);
        let id2 = self.get_tile_at(position2.x, position2.y);
        if let Some(id) = id1 {
            if id.solid { return true }
        }
        if let Some(id) = id2 {
            if id.solid { return true }
        }
        return false
    }

}

impl Drawable for Map {
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
                    let scalex = (MAP_TILE_SIZE as Transformation / image.get_width() as Transformation) as Transformation;
                    let scaley = (MAP_TILE_SIZE as Transformation / image.get_height() as Transformation) as Transformation;
                    graphics.draw_full(
                        &image, 
                        (pos_c as Position * MAP_TILE_SIZE as Position), 
                        (pos_l as Position * MAP_TILE_SIZE as Position), 
                        0., scalex, scaley, 0., 0. 
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
