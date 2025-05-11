use std::collections::HashMap;
use crate::position::Position;
use crate::tile::Tile;
use crate::error::OtmbError;
use crate::house::HouseManager;

/// Represents a game map loaded from an OTMB file
#[derive(Debug)]
pub struct Map {
    /// Map width in tiles
    pub width: u16,
    
    /// Map height in tiles
    pub height: u16,
    
    /// Map description
    pub description: String,
    
    /// OTMB version
    pub version: u32,
    
    /// Map tiles, indexed by position
    tiles: HashMap<Position, Tile>,
    
    /// House manager for this map
    house_manager: Option<HouseManager>,
}

impl Map {
    /// Create a new empty map with the given dimensions
    pub fn new(width: u16, height: u16, description: String, version: u32) -> Self {
        Self {
            width,
            height,
            description,
            version,
            tiles: HashMap::new(),
            house_manager: None,
        }
    }
    
    /// Add a tile to the map
    pub fn add_tile(&mut self, tile: Tile) -> Result<(), OtmbError> {
        let pos = tile.position;
        
        // Validate position is within map bounds
        if pos.x >= self.width || pos.y >= self.height {
            return Err(OtmbError::InvalidDimensions(
                format!("Position ({}, {}, {}) is outside map bounds", pos.x, pos.y, pos.z)
            ));
        }
        
        self.tiles.insert(pos, tile);
        Ok(())
    }
    
    /// Get a tile at the specified position
    pub fn get_tile(&self, position: &Position) -> Option<&Tile> {
        self.tiles.get(position)
    }
    
    /// Get a mutable reference to a tile at the specified position
    pub fn get_tile_mut(&mut self, position: &Position) -> Option<&mut Tile> {
        self.tiles.get_mut(position)
    }
    
    /// Check if a position is walkable
    pub fn is_walkable(&self, position: &Position) -> bool {
        self.get_tile(position)
            .map(|tile| tile.is_walkable())
            .unwrap_or(false)
    }
    
    /// Get all tiles in the map
    pub fn tiles(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.values()
    }
    
    /// Get the number of tiles in the map
    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }
    
    /// Set the house manager for this map
    pub fn set_house_manager(&mut self, house_manager: HouseManager) {
        self.house_manager = Some(house_manager);
    }
    
    /// Get the house manager for this map
    pub fn house_manager(&self) -> Option<&HouseManager> {
        self.house_manager.as_ref()
    }
    
    /// Get a mutable reference to the house manager for this map
    pub fn house_manager_mut(&mut self) -> Option<&mut HouseManager> {
        self.house_manager.as_mut()
    }
    
    /// Get the number of houses in the map
    pub fn house_count(&self) -> usize {
        self.house_manager.as_ref().map_or(0, |hm| hm.house_count())
    }
}