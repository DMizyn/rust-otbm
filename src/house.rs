use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::error::OtmbError;
use crate::position::Position;
use crate::Result;

/// Represents a house in the game world
#[derive(Debug, Clone)]
pub struct House {
    /// Unique identifier for the house
    pub id: u32,
    
    /// Name of the house
    pub name: String,
    
    /// Entry position (where players enter the house)
    pub entry: Position,
    
    /// Rent price
    pub rent: u32,
    
    /// Town ID this house belongs to
    pub town_id: u32,
    
    /// Size of the house in tiles
    pub size: u32,
    
    /// Whether this is a guild hall
    pub guild_hall: bool,
    
    /// Tiles that belong to this house
    pub tiles: HashSet<Position>,
    
    /// Doors that belong to this house
    pub doors: Vec<Door>,
    
    /// Access list (players who can enter)
    pub access_list: Vec<String>,
}

/// Represents a door in a house
#[derive(Debug, Clone)]
pub struct Door {
    /// Position of the door
    pub position: Position,
    
    /// Door ID
    pub id: u16,
    
    /// Whether the door is locked
    pub locked: bool,
}

/// Collection of houses in the game world
#[derive(Debug, Clone)]
pub struct HouseManager {
    /// Houses indexed by ID
    houses: HashMap<u32, House>,
}

impl HouseManager {
    /// Create a new empty house manager
    pub fn new() -> Self {
        Self {
            houses: HashMap::new(),
        }
    }
    
    /// Load houses from an XML file
    pub fn load_from_xml<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        Self::parse_xml(&content)
    }
    
    /// Parse house data from XML content
    fn parse_xml(content: &str) -> Result<Self> {
        let mut manager = Self::new();
        
        // Simple XML parsing without external dependencies
        // In a real implementation, you would use a proper XML parser
        
        for line in content.lines() {
            if line.contains("<house ") {
                // Extract house attributes
                let house = Self::parse_house_element(line)?;
                manager.add_house(house);
            }
        }
        
        Ok(manager)
    }
    
    /// Parse a house element from XML
    fn parse_house_element(line: &str) -> Result<House> {
        // Extract attributes using simple string operations
        // In a real implementation, you would use a proper XML parser
        
        let name = Self::extract_attribute(line, "name")
            .ok_or_else(|| OtmbError::InvalidFormat("Missing house name".to_string()))?;
        
        let id = Self::extract_attribute(line, "houseid")
            .and_then(|s| s.parse::<u32>().ok())
            .ok_or_else(|| OtmbError::InvalidFormat("Invalid house ID".to_string()))?;
        
        let entry_x = Self::extract_attribute(line, "entryx")
            .and_then(|s| s.parse::<u16>().ok())
            .ok_or_else(|| OtmbError::InvalidFormat("Invalid entry X".to_string()))?;
        
        let entry_y = Self::extract_attribute(line, "entryy")
            .and_then(|s| s.parse::<u16>().ok())
            .ok_or_else(|| OtmbError::InvalidFormat("Invalid entry Y".to_string()))?;
        
        let entry_z = Self::extract_attribute(line, "entryz")
            .and_then(|s| s.parse::<u8>().ok())
            .ok_or_else(|| OtmbError::InvalidFormat("Invalid entry Z".to_string()))?;
        
        let rent = Self::extract_attribute(line, "rent")
            .and_then(|s| s.parse::<u32>().ok())
            .ok_or_else(|| OtmbError::InvalidFormat("Invalid rent".to_string()))?;
        
        let town_id = Self::extract_attribute(line, "townid")
            .and_then(|s| s.parse::<u32>().ok())
            .ok_or_else(|| OtmbError::InvalidFormat("Invalid town ID".to_string()))?;
        
        let size = Self::extract_attribute(line, "size")
            .and_then(|s| s.parse::<u32>().ok())
            .ok_or_else(|| OtmbError::InvalidFormat("Invalid size".to_string()))?;
        
        // Check if this is a guild hall
        let guild_hall = Self::extract_attribute(line, "guildhall")
            .map(|s| s == "true" || s == "yes" || s == "1")
            .unwrap_or(false);
        
        Ok(House {
            id,
            name,
            entry: Position::new(entry_x, entry_y, entry_z),
            rent,
            town_id,
            size,
            guild_hall,
            tiles: HashSet::new(),
            doors: Vec::new(),
            access_list: Vec::new(),
        })
    }
    
    /// Extract an attribute value from an XML element
    fn extract_attribute(line: &str, attr_name: &str) -> Option<String> {
        let attr_prefix = format!("{}=\"", attr_name);
        if let Some(start_pos) = line.find(&attr_prefix) {
            let start = start_pos + attr_prefix.len();
            if let Some(end_pos) = line[start..].find('"') {
                return Some(line[start..start + end_pos].to_string());
            }
        }
        None
    }
    
    /// Add a house to the manager
    pub fn add_house(&mut self, house: House) {
        self.houses.insert(house.id, house);
    }
    
    /// Get a house by ID
    pub fn get_house(&self, id: u32) -> Option<&House> {
        self.houses.get(&id)
    }
    
    /// Get a mutable reference to a house by ID
    pub fn get_house_mut(&mut self, id: u32) -> Option<&mut House> {
        self.houses.get_mut(&id)
    }
    
    /// Get all houses
    pub fn houses(&self) -> impl Iterator<Item = &House> {
        self.houses.values()
    }
    
    /// Get the number of houses
    pub fn house_count(&self) -> usize {
        self.houses.len()
    }
    
    /// Find a house by position
    pub fn find_house_by_position(&self, pos: &Position) -> Option<&House> {
        self.houses.values().find(|house| house.tiles.contains(pos))
    }
    
    /// Add a tile to a house
    pub fn add_tile_to_house(&mut self, house_id: u32, pos: Position) -> Result<()> {
        if let Some(house) = self.houses.get_mut(&house_id) {
            house.tiles.insert(pos);
            Ok(())
        } else {
            Err(OtmbError::InvalidFormat(format!("House with ID {} not found", house_id)))
        }
    }
    
    /// Add a door to a house
    pub fn add_door_to_house(&mut self, house_id: u32, door: Door) -> Result<()> {
        if let Some(house) = self.houses.get_mut(&house_id) {
            house.doors.push(door);
            Ok(())
        } else {
            Err(OtmbError::InvalidFormat(format!("House with ID {} not found", house_id)))
        }
    }
    
    /// Add a player to a house's access list
    pub fn add_player_to_access_list(&mut self, house_id: u32, player_name: String) -> Result<()> {
        if let Some(house) = self.houses.get_mut(&house_id) {
            house.access_list.push(player_name);
            Ok(())
        } else {
            Err(OtmbError::InvalidFormat(format!("House with ID {} not found", house_id)))
        }
    }
    
    /// Save houses to an XML file
    pub fn save_to_xml<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut content = String::from("<?xml version=\"1.0\"?>\n<houses>\n");
        
        for house in self.houses.values() {
            let guild_hall_attr = if house.guild_hall { " guildhall=\"true\"" } else { "" };
            
            content.push_str(&format!("\t<house name=\"{}\" houseid=\"{}\" entryx=\"{}\" entryy=\"{}\" entryz=\"{}\" rent=\"{}\"{}  townid=\"{}\" size=\"{}\" />\n",
                house.name,
                house.id,
                house.entry.x,
                house.entry.y,
                house.entry.z,
                house.rent,
                guild_hall_attr,
                house.town_id,
                house.size
            ));
        }
        
        content.push_str("</houses>\n");
        
        std::fs::write(path, content)?;
        
        Ok(())
    }
}