use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

use crate::error::OtmbError;
use crate::map::Map;
use crate::position::Position;
use crate::tile::Tile;
use crate::item::{Item, ItemAttribute, AttributeValue};

/// Magic bytes that identify an OTBM file
const OTBM_IDENTIFIER: &[u8] = b"OTBM";

/// Supported OTBM versions
const SUPPORTED_VERSIONS: &[u32] = &[1, 2, 3];

/// Attribute types as defined in The Forgotten Server
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AttributeType {
    Description = 1,
    ExtId = 2,
    TileFlags = 3,
    ActionId = 4,
    UniqueId = 5,
    Text = 6,
    Desc = 7,
    TeleportDestination = 8,
    Item = 9,
    DepotId = 10,
    ExpId = 11,
    Duration = 12,
    DecayingState = 13,
    Name = 14,
    Article = 15,
    Plural = 16,
    Weight = 17,
    Attack = 18,
    Defense = 19,
    ExtraDefense = 20,
    Armor = 21,
    HitChance = 22,
    ShootRange = 23,
    CustomAttribute = 24,
    Charges = 25,
    FluidType = 26,
    Capacity = 27,
    ContainerType = 28,
    SpriteId = 29,
    AnimationId = 30,
    CorpseId = 31,
    Blocksolid = 32,
    Blockprojectile = 33,
    Blockpathfind = 34,
    Protectionzone = 35,
    Movable = 36,
    Pickupable = 37,
    Hangable = 38,
    Vertical = 39,
    Horizontal = 40,
    Rotatable = 41,
    Readable = 42,
    Stackable = 43,
    FluidSource = 44,
    FluidContainer = 45,
    DoorId = 46,
    RuneSpell = 47,
    LightLevel = 48,
    LightColor = 49,
    DecayTo = 50,
    DecayTime = 51,
    MinimapColor = 52,
    TradeAs = 53,
    ShowCount = 54,
    WrapableTo = 55,
    UnwrapableTo = 56,
    TopOrder = 57,
}

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

/// Loader for OTBM map files
pub struct OtmbLoader;

impl OtmbLoader {
    /// Load an OTBM file from the given path
    pub fn load<P: AsRef<Path>>(path: P) -> crate::Result<Map> {
        let mut file = File::open(path)?;
        
        // Check file identifier
        let mut identifier = [0u8; 4];
        file.read_exact(&mut identifier)?;
        
        if &identifier != OTBM_IDENTIFIER {
            return Err(OtmbError::InvalidFormat(
                format!("File is not a valid OTBM file. Expected {:?}, got {:?}", 
                    OTBM_IDENTIFIER, identifier)
            ));
        }
        
        // Read version
        let version = file.read_u32::<LittleEndian>()?;
        
        if !SUPPORTED_VERSIONS.contains(&version) {
            return Err(OtmbError::UnsupportedVersion(version));
        }
        
        // Read node type - should be root
        let node_type = file.read_u8()?;
        if node_type != OtbmNodeType::RootV1 as u8 {
            return Err(OtmbError::InvalidFormat(
                format!("Expected root node type {}, got {}", 
                    OtbmNodeType::RootV1 as u8, node_type)
            ));
        }
        
        // Skip 3 unknown bytes (usually 0)
        let mut unknown = [0u8; 3];
        file.read_exact(&mut unknown)?;
        
        // Read map header
        let header_size = file.read_u32::<LittleEndian>()?;
        let mut header_data = vec![0u8; header_size as usize];
        file.read_exact(&mut header_data)?;
        
        // Parse header data
        let (width, height, description) = Self::parse_header(&header_data)?;
        
        // Create map
        let mut map = Map::new(width, height, description, version);
        
        // Read map data node
        let node_type = file.read_u8()?;
        if node_type != OtbmNodeType::MapData as u8 {
            return Err(OtmbError::InvalidFormat(
                format!("Expected map data node type {}, got {}", 
                    OtbmNodeType::MapData as u8, node_type)
            ));
        }
        
        // Read map data
        Self::read_map_data(&mut file, &mut map)?;
        
        Ok(map)
    }
    
    /// Parse the map header
    fn parse_header(data: &[u8]) -> crate::Result<(u16, u16, String)> {
        // In a real implementation, you would parse the header data
        // to extract map dimensions and description
        // For now, we'll use placeholder values
        let width = 1000;
        let height = 1000;
        let description = "Map loaded from OTBM file".to_string();
        
        Ok((width, height, description))
    }
    
    /// Read map data from the OTBM file
    fn read_map_data(file: &mut File, map: &mut Map) -> crate::Result<()> {
        // In a real implementation, you would read the map data
        // including tiles, items, and their attributes
        // For now, we'll create some placeholder tiles
        
        // Create a few test tiles
        for x in 0..10 {
            for y in 0..10 {
                let position = Position::new(x, y, 7);
                let mut tile = Tile::new(position);
                
                // Add a ground item
                let mut ground = Item::new(100);
                tile.add_item(ground);
                
                // Add the tile to the map
                map.add_tile(tile)?;
            }
        }
        
        Ok(())
    }
    
    /// Save a map to an OTBM file
    pub fn save<P: AsRef<Path>>(map: &Map, path: P) -> crate::Result<()> {
        let mut file = File::create(path)?;
        
        // Write file identifier
        file.write_all(OTBM_IDENTIFIER)?;
        
        // Write version
        file.write_u32::<LittleEndian>(map.version)?;
        
        // Write root node type
        file.write_u8(OtbmNodeType::RootV1 as u8)?;
        
        // Write 3 unknown bytes (usually 0)
        file.write_all(&[0, 0, 0])?;
        
        // Write header size and data
        let header_data = Self::create_header_data(map)?;
        file.write_u32::<LittleEndian>(header_data.len() as u32)?;
        file.write_all(&header_data)?;
        
        // Write map data node type
        file.write_u8(OtbmNodeType::MapData as u8)?;
        
        // Write map data
        Self::write_map_data(&mut file, map)?;
        
        Ok(())
    }
    
    /// Create header data for the OTBM file
    fn create_header_data(map: &Map) -> crate::Result<Vec<u8>> {
        // In a real implementation, you would create the header data
        // including map dimensions and description
        // For now, we'll return an empty vector
        Ok(Vec::new())
    }
    
    /// Write map data to the OTBM file
    fn write_map_data(file: &mut File, map: &Map) -> crate::Result<()> {
        // In a real implementation, you would write the map data
        // including tiles, items, and their attributes
        // For now, we'll do nothing
        Ok(())
    }
}