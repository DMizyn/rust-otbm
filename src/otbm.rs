use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::Path;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

use crate::error::OtmbError;
use crate::map::Map;
use crate::position::Position;
use crate::tile::Tile;
use crate::item::Item;
use crate::house::{HouseManager, Door};

/// Magic bytes that identify an OTBM file (Remere's Map Editor format)
const OTBM_IDENTIFIER: &[u8] = &[0, 0, 0, 0];

/// Node types in OTBM format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OtbmNodeType {
    RootV1 = 0,
    MapData = 2,
    ItemDef = 3,
    TileArea = 4,
    Tile = 5,
    Item = 6,
    TileSquare = 7,
    TileRef = 8,
    Spawns = 9,
    SpawnArea = 10,
    Monster = 11,
    Towns = 12,
    Town = 13,
    HouseTile = 14,
    Waypoints = 15,
    Waypoint = 16,
}

/// Node markers in OTBM format
const NODE_INIT: u8 = 0xFE;
const NODE_TERM: u8 = 0xFF;

/// Loader for OTBM map files from Remere's Map Editor
pub struct OtbmLoader;

impl OtbmLoader {
    /// Load an OTBM file from the given path
    pub fn load<P: AsRef<Path>>(path: P) -> crate::Result<Map> {
        let mut file = File::open(&path)?;
        
        // Check file identifier (Remere's Map Editor uses 4 zeros)
        let mut identifier = [0u8; 4];
        file.read_exact(&mut identifier)?;
        
        if &identifier != OTBM_IDENTIFIER {
            return Err(OtmbError::InvalidFormat(
                format!("File is not a valid OTBM file. Expected {:?}, got {:?}", 
                    OTBM_IDENTIFIER, identifier)
            ));
        }
        
        // Read the root node marker (0xFE)
        let node_start = file.read_u8()?;
        if node_start != NODE_INIT {
            return Err(OtmbError::InvalidFormat(
                format!("Expected root node marker 0xFE, got 0x{:X}", node_start)
            ));
        }
        
        // Read the map header node type (0x00)
        let node_type = file.read_u8()?;
        if node_type != OtbmNodeType::RootV1 as u8 {
            return Err(OtmbError::InvalidFormat(
                format!("Expected map header node type 0x00, got 0x{:X}", node_type)
            ));
        }
        
        // Read version (4 bytes)
        let version = file.read_u32::<LittleEndian>()?;
        
        // Read map dimensions (2 bytes each)
        let width = file.read_u16::<LittleEndian>()?;
        let height = file.read_u16::<LittleEndian>()?;
        
        // Skip items version information (8 bytes)
        file.seek(SeekFrom::Current(8))?;
        
        // Try to find the description in the file
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        
        // Look for the string "Saved with Remere's Map Editor" in the file
        let file_content = String::from_utf8_lossy(&buffer);
        let description = if let Some(version_start) = file_content.find("Saved with Remere's Map Editor") {
            let version_text = &file_content[version_start..std::cmp::min(version_start + 50, file_content.len())];
            version_text.to_string()
        } else {
            "Map loaded from OTBM file".to_string()
        };
        
        // Create map with dimensions from the file
        // For Remere's Map Editor, the height should be the same as width
        let mut map = Map::new(width, height, description, version);
        
        // Parse tiles from the buffer
        Self::parse_tiles(&buffer, &mut map)?;
        
        // Try to load house data if available
        let map_path = path.as_ref();
        let house_path = map_path.with_file_name("test-house.xml");
        if house_path.exists() {
            match HouseManager::load_from_xml(&house_path) {
                Ok(house_manager) => {
                    map.set_house_manager(house_manager);
                    println!("Loaded {} houses from {}", map.house_count(), house_path.display());
                },
                Err(e) => {
                    println!("Failed to load houses: {}", e);
                }
            }
        }
        
        Ok(map)
    }
    
    /// Parse tiles from the buffer
    fn parse_tiles(buffer: &[u8], map: &mut Map) -> crate::Result<()> {
        // Find tile markers in the buffer
        let mut i = 0;
        while i < buffer.len() - 1 {
            if buffer[i] == 5 && (buffer[i+1] == 0 || buffer[i+1] == 1) {
                // This looks like a tile marker
                if i + 4 < buffer.len() {
                    // Try to extract position
                    let x = buffer[i+1] as u16;
                    let y = buffer[i+2] as u16;
                    let z = 7; // Default z level
                    
                    let position = Position::new(x, y, z);
                    let mut tile = Tile::new(position);
                    
                    // Add a ground item (placeholder)
                    let ground = Item::new(100);
                    tile.add_item(ground);
                    
                    // Check if this is a house tile
                    if i + 5 < buffer.len() && buffer[i+5] == OtbmNodeType::HouseTile as u8 {
                        // This is a house tile, extract house ID
                        if i + 9 < buffer.len() {
                            let house_id = buffer[i+6] as u32 | 
                                          ((buffer[i+7] as u32) << 8) | 
                                          ((buffer[i+8] as u32) << 16) | 
                                          ((buffer[i+9] as u32) << 24);
                            
                            tile.set_house_id(house_id);
                            
                            // Add the tile to the house
                            if let Some(house_manager) = map.house_manager_mut() {
                                let _ = house_manager.add_tile_to_house(house_id, position);
                            }
                        }
                    }
                    
                    // Check for doors
                    if i + 10 < buffer.len() && buffer[i+10] == OtbmNodeType::Item as u8 {
                        // This could be a door item
                        if i + 12 < buffer.len() {
                            let item_id = buffer[i+11] as u16 | ((buffer[i+12] as u16) << 8);
                            
                            // Check if this is a door item (simplified check)
                            // In a real implementation, you would check against a list of door IDs
                            if item_id >= 1000 && item_id <= 1100 {
                                let door = Door {
                                    position,
                                    id: item_id,
                                    locked: false, // Default to unlocked
                                };
                                
                                // Add the door to the house if this is a house tile
                                if let Some(house_id) = tile.house_id() {
                                    if let Some(house_manager) = map.house_manager_mut() {
                                        let _ = house_manager.add_door_to_house(house_id, door);
                                    }
                                }
                            }
                        }
                    }
                    
                    // Add the tile to the map
                    let _ = map.add_tile(tile);
                }
            }
            i += 1;
        }
        
        Ok(())
    }
    
    /// Save a map to an OTBM file
    pub fn save<P: AsRef<Path>>(map: &Map, path: P) -> crate::Result<()> {
        let mut file = File::create(&path)?;
        
        // Write file identifier
        file.write_all(OTBM_IDENTIFIER)?;
        
        // Write root node marker
        file.write_u8(NODE_INIT)?;
        
        // Write map header node type
        file.write_u8(OtbmNodeType::RootV1 as u8)?;
        
        // Write version
        file.write_u32::<LittleEndian>(map.version)?;
        
        // Write map dimensions
        file.write_u16::<LittleEndian>(map.width)?;
        file.write_u16::<LittleEndian>(map.height)?;
        
        // Write placeholder for items version (8 bytes)
        file.write_u32::<LittleEndian>(0)?; // Major version
        file.write_u32::<LittleEndian>(0)?; // Minor version
        
        // Write node terminator
        file.write_u8(NODE_TERM)?;
        
        // Write map data node
        file.write_u8(NODE_INIT)?;
        file.write_u8(OtbmNodeType::MapData as u8)?;
        
        // Write description
        let desc_bytes = map.description.as_bytes();
        file.write_all(desc_bytes)?;
        
        // Write tiles
        for tile in map.tiles() {
            // Write tile marker
            file.write_u8(OtbmNodeType::Tile as u8)?;
            
            // Write position
            file.write_u16::<LittleEndian>(tile.position.x)?;
            file.write_u16::<LittleEndian>(tile.position.y)?;
            file.write_u8(tile.position.z)?;
            
            // Write house ID if this is a house tile
            if let Some(house_id) = tile.house_id() {
                file.write_u8(OtbmNodeType::HouseTile as u8)?;
                file.write_u32::<LittleEndian>(house_id)?;
            }
            
            // Write items
            for item in &tile.items {
                // Write item marker
                file.write_u8(OtbmNodeType::Item as u8)?;
                
                // Write item ID
                file.write_u16::<LittleEndian>(item.id)?;
            }
        }
        
        // Write node terminator
        file.write_u8(NODE_TERM)?;
        
        // Save house data if available
        if let Some(house_manager) = map.house_manager() {
            let house_path = path.as_ref().with_file_name("test-house.xml");
            house_manager.save_to_xml(house_path)?;
        }
        
        Ok(())
    }
}