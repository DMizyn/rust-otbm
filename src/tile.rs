use crate::item::{Item, ItemAttribute};
use crate::position::Position;

/// Represents a tile in the game world
#[derive(Debug, Clone)]
pub struct Tile {
    /// Position of the tile in the world
    pub position: Position,
    
    /// Items on this tile (ground item is at index 0)
    pub items: Vec<Item>,
    
    /// Flags for the tile
    pub flags: TileFlags,
    
    /// House ID if this tile belongs to a house
    house_id: Option<u32>,
}

/// Flags that can be applied to a tile
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileFlags {
    /// Whether the tile blocks movement
    pub blocking: bool,
    
    /// Whether the tile is protected (PvP restrictions)
    pub protected: bool,
    
    /// Whether the tile has special properties
    pub special: bool,
    
    /// Whether the tile is a teleport
    pub teleport: bool,
    
    /// Whether the tile is a house tile
    pub house: bool,
    
    /// Whether the tile is a depot
    pub depot: bool,
    
    /// Whether the tile is a quest zone
    pub quest_zone: bool,
    
    /// Whether the tile is a no-logout zone
    pub no_logout: bool,
    
    /// Whether the tile is a PVP zone
    pub pvp_zone: bool,
    
    /// Whether the tile is a no-combat zone
    pub no_combat: bool,
}

impl Default for TileFlags {
    fn default() -> Self {
        Self {
            blocking: false,
            protected: false,
            special: false,
            teleport: false,
            house: false,
            depot: false,
            quest_zone: false,
            no_logout: false,
            pvp_zone: false,
            no_combat: false,
        }
    }
}

impl Tile {
    /// Create a new empty tile at the given position
    pub fn new(position: Position) -> Self {
        Self {
            position,
            items: Vec::new(),
            flags: TileFlags::default(),
            house_id: None,
        }
    }
    
    /// Add an item to this tile
    pub fn add_item(&mut self, item: Item) {
        // Check if the item is a teleport
        if item.has_attribute(&ItemAttribute::TeleportDestination) {
            self.flags.teleport = true;
        }
        
        // Check if the item is a protection zone item
        if item.get_boolean(&ItemAttribute::Protectionzone).unwrap_or(false) {
            self.flags.protected = true;
        }
        
        self.items.push(item);
        
        // Update tile flags based on items
        self.update_flags();
    }
    
    /// Update tile flags based on contained items
    fn update_flags(&mut self) {
        // Check if any item blocks movement
        self.flags.blocking = self.items.iter().any(|item| item.is_blocking());
        
        // Check for special items
        self.flags.special = self.items.iter().any(|item| {
            item.is_teleport() || item.is_door() || item.is_container()
        });
        
        // Update house flag
        self.flags.house = self.house_id.is_some();
    }
    
    /// Get the ground item (bottom-most item) if any
    pub fn ground_item(&self) -> Option<&Item> {
        self.items.first()
    }
    
    /// Check if the tile is walkable
    pub fn is_walkable(&self) -> bool {
        !self.flags.blocking
    }
    
    /// Check if the tile has a teleport
    pub fn has_teleport(&self) -> bool {
        self.flags.teleport
    }
    
    /// Check if the tile is protected
    pub fn is_protected(&self) -> bool {
        self.flags.protected
    }
    
    /// Get the teleport destination if this tile has a teleport
    pub fn teleport_destination(&self) -> Option<Position> {
        for item in &self.items {
            if let Some((x, y, z)) = item.get_position(&ItemAttribute::TeleportDestination) {
                return Some(Position::new(x, y, z));
            }
        }
        None
    }
    
    /// Get all items of a specific type on this tile
    pub fn items_by_id(&self, id: u16) -> Vec<&Item> {
        self.items.iter().filter(|item| item.id == id).collect()
    }
    
    /// Check if the tile has an item with a specific ID
    pub fn has_item_with_id(&self, id: u16) -> bool {
        self.items.iter().any(|item| item.id == id)
    }
    
    /// Get the top item on this tile
    pub fn top_item(&self) -> Option<&Item> {
        self.items.last()
    }
    
    /// Get the number of items on this tile
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
    
    /// Remove the top item from this tile
    pub fn remove_top_item(&mut self) -> Option<Item> {
        let item = self.items.pop();
        self.update_flags();
        item
    }
    
    /// Remove an item at a specific index
    pub fn remove_item_at(&mut self, index: usize) -> Option<Item> {
        if index < self.items.len() {
            let item = self.items.remove(index);
            self.update_flags();
            Some(item)
        } else {
            None
        }
    }
    
    /// Set the house ID for this tile
    pub fn set_house_id(&mut self, house_id: u32) {
        self.house_id = Some(house_id);
        self.flags.house = true;
    }
    
    /// Get the house ID for this tile
    pub fn house_id(&self) -> Option<u32> {
        self.house_id
    }
    
    /// Check if this tile belongs to a house
    pub fn is_house_tile(&self) -> bool {
        self.house_id.is_some()
    }
}