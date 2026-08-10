use byteorder::{LittleEndian, WriteBytesExt};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use crate::error::OtmbError;
use crate::house::{Door, HouseManager};
use crate::item::Item;
use crate::map::Map;
use crate::position::Position;
use crate::tile::Tile;

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
const NODE_ESCAPE: u8 = 0xFD;

#[derive(Debug)]
struct OtbmNode {
    node_type: u8,
    props: Vec<u8>,
    children: Vec<OtbmNode>,
}

impl OtbmNode {
    fn parse(data: &[u8], cursor: &mut usize) -> crate::Result<Self> {
        if data.get(*cursor) != Some(&NODE_INIT) {
            return Err(OtmbError::InvalidFormat(format!(
                "Expected node start at offset {}",
                cursor
            )));
        }
        *cursor += 1;

        let node_type = *data.get(*cursor).ok_or_else(|| {
            OtmbError::InvalidFormat("Missing node type at end of file".to_string())
        })?;
        *cursor += 1;

        let mut props = Vec::new();
        let mut children = Vec::new();
        loop {
            let byte = *data.get(*cursor).ok_or_else(|| {
                OtmbError::InvalidFormat(format!("Unterminated node type {}", node_type))
            })?;

            match byte {
                NODE_ESCAPE => {
                    *cursor += 1;
                    let escaped = *data.get(*cursor).ok_or_else(|| {
                        OtmbError::InvalidFormat("Escape marker at end of file".to_string())
                    })?;
                    props.push(escaped);
                    *cursor += 1;
                }
                NODE_INIT => children.push(Self::parse(data, cursor)?),
                NODE_TERM => {
                    *cursor += 1;
                    return Ok(Self {
                        node_type,
                        props,
                        children,
                    });
                }
                _ => {
                    props.push(byte);
                    *cursor += 1;
                }
            }
        }
    }
}

fn read_u16(data: &[u8], offset: usize) -> crate::Result<u16> {
    let bytes = data.get(offset..offset + 2).ok_or_else(|| {
        OtmbError::InvalidFormat(format!("Missing u16 at property offset {}", offset))
    })?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32(data: &[u8], offset: usize) -> crate::Result<u32> {
    let bytes = data.get(offset..offset + 4).ok_or_else(|| {
        OtmbError::InvalidFormat(format!("Missing u32 at property offset {}", offset))
    })?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_string(data: &[u8], cursor: &mut usize) -> crate::Result<String> {
    let length = read_u16(data, *cursor)? as usize;
    *cursor += 2;
    let bytes = data.get(*cursor..*cursor + length).ok_or_else(|| {
        OtmbError::InvalidFormat(format!(
            "String of length {} exceeds node properties",
            length
        ))
    })?;
    *cursor += length;
    Ok(String::from_utf8_lossy(bytes).into_owned())
}

/// Loader for OTBM map files from Remere's Map Editor
pub struct OtbmLoader;

impl OtbmLoader {
    /// Load an OTBM file from the given path
    pub fn load<P: AsRef<Path>>(path: P) -> crate::Result<Map> {
        let mut data = Vec::new();
        File::open(&path)?.read_to_end(&mut data)?;

        if data.len() < OTBM_IDENTIFIER.len() + 3 {
            return Err(OtmbError::InvalidFormat("File is too short".to_string()));
        }

        if &data[..OTBM_IDENTIFIER.len()] != OTBM_IDENTIFIER {
            return Err(OtmbError::InvalidFormat(format!(
                "File is not a valid OTBM file. Expected {:?}, got {:?}",
                OTBM_IDENTIFIER,
                &data[..OTBM_IDENTIFIER.len()]
            )));
        }

        let mut cursor = OTBM_IDENTIFIER.len();
        let root = OtbmNode::parse(&data, &mut cursor)?;
        if cursor != data.len() {
            return Err(OtmbError::InvalidFormat(format!(
                "Unexpected bytes after root node at offset {}",
                cursor
            )));
        }

        if root.node_type != OtbmNodeType::RootV1 as u8 {
            return Err(OtmbError::InvalidFormat(format!(
                "Expected map header node type 0x00, got 0x{:X}",
                root.node_type
            )));
        }

        if root.props.len() < 16 {
            return Err(OtmbError::InvalidFormat(format!(
                "Root header is too short: {} bytes",
                root.props.len()
            )));
        }

        let version = read_u32(&root.props, 0)?;
        let width = read_u16(&root.props, 4)?;
        let height = read_u16(&root.props, 6)?;
        let map_data = root
            .children
            .iter()
            .find(|node| node.node_type == OtbmNodeType::MapData as u8)
            .ok_or_else(|| OtmbError::MissingData("OTBM map-data node".to_string()))?;
        let description = Self::parse_map_description(&map_data.props)?;

        let mut map = Map::new(width, height, description, version);
        Self::parse_tiles(map_data, &mut map)?;

        // Try to load house data if available
        let map_path = path.as_ref();
        let house_file_name = map_path
            .file_stem()
            .map(|stem| format!("{}-house.xml", stem.to_string_lossy()))
            .unwrap_or_else(|| "world-house.xml".to_string());
        let house_path = map_path.with_file_name(house_file_name);
        if house_path.exists() {
            match HouseManager::load_from_xml(&house_path) {
                Ok(house_manager) => {
                    map.set_house_manager(house_manager);
                    println!(
                        "Loaded {} houses from {}",
                        map.house_count(),
                        house_path.display()
                    );
                }
                Err(e) => {
                    println!("Failed to load houses: {}", e);
                }
            }
        }

        Ok(map)
    }

    fn parse_map_description(props: &[u8]) -> crate::Result<String> {
        let mut cursor = 0;
        let mut description = "Map loaded from OTBM file".to_string();
        while cursor < props.len() {
            let attribute = props[cursor];
            cursor += 1;
            match attribute {
                1 | 11 | 13 => {
                    let value = read_string(props, &mut cursor)?;
                    if attribute == 1 {
                        description = value;
                    }
                }
                other => {
                    return Err(OtmbError::InvalidFormat(format!(
                        "Unknown map attribute {}",
                        other
                    )));
                }
            }
        }
        Ok(description)
    }

    /// Parse tile-area nodes while respecting OTBM node nesting and escaping.
    fn parse_tiles(map_data: &OtbmNode, map: &mut Map) -> crate::Result<()> {
        for area in map_data
            .children
            .iter()
            .filter(|node| node.node_type == OtbmNodeType::TileArea as u8)
        {
            if area.props.len() < 5 {
                return Err(OtmbError::InvalidFormat(
                    "Tile-area coordinates are incomplete".to_string(),
                ));
            }

            let area_x = read_u16(&area.props, 0)?;
            let area_y = read_u16(&area.props, 2)?;
            let area_z = area.props[4];
            if area_z > 14 {
                return Err(OtmbError::InvalidFormat(format!(
                    "Tile area uses invalid z-level {}",
                    area_z
                )));
            }

            for tile_node in &area.children {
                let is_house_tile = tile_node.node_type == OtbmNodeType::HouseTile as u8;
                if tile_node.node_type != OtbmNodeType::Tile as u8 && !is_house_tile {
                    return Err(OtmbError::InvalidFormat(format!(
                        "Unexpected node type {} in tile area",
                        tile_node.node_type
                    )));
                }
                if tile_node.props.len() < 2 {
                    return Err(OtmbError::InvalidFormat(
                        "Tile coordinates are incomplete".to_string(),
                    ));
                }

                let x = area_x
                    .checked_add(tile_node.props[0] as u16)
                    .ok_or_else(|| {
                        OtmbError::InvalidDimensions("Tile x coordinate overflow".to_string())
                    })?;
                let y = area_y
                    .checked_add(tile_node.props[1] as u16)
                    .ok_or_else(|| {
                        OtmbError::InvalidDimensions("Tile y coordinate overflow".to_string())
                    })?;
                if x >= map.width || y >= map.height {
                    log::warn!(
                        "Skipping OTBM tile outside map bounds at ({}, {}, {})",
                        x,
                        y,
                        area_z
                    );
                    continue;
                }
                let position = Position::new(x, y, area_z);
                let mut tile = Tile::new(position);
                let mut prop_cursor = 2;
                let house_id = if is_house_tile {
                    let id = read_u32(&tile_node.props, prop_cursor)?;
                    prop_cursor += 4;
                    tile.set_house_id(id);
                    Some(id)
                } else {
                    None
                };

                while prop_cursor < tile_node.props.len() {
                    let attribute = tile_node.props[prop_cursor];
                    prop_cursor += 1;
                    match attribute {
                        3 => {
                            let flags = read_u32(&tile_node.props, prop_cursor)?;
                            prop_cursor += 4;
                            tile.flags.protected = flags & (1 << 0) != 0;
                            tile.flags.no_combat = flags & (1 << 2) != 0;
                            tile.flags.no_logout = flags & (1 << 3) != 0;
                            tile.flags.pvp_zone = flags & (1 << 4) != 0;
                        }
                        9 => {
                            let item_id = read_u16(&tile_node.props, prop_cursor)?;
                            prop_cursor += 2;
                            tile.add_item(Item::new(item_id));
                        }
                        // Item-specific attributes after an inline ground item require
                        // items.otb metadata to determine their encoded size. The map used
                        // by this project stores the ground as the final inline attribute.
                        _ => break,
                    }
                }

                for item_node in &tile_node.children {
                    if item_node.node_type != OtbmNodeType::Item as u8 || item_node.props.len() < 2
                    {
                        return Err(OtmbError::InvalidFormat(format!(
                            "Invalid item node at ({}, {}, {})",
                            x, y, area_z
                        )));
                    }
                    let item_id = read_u16(&item_node.props, 0)?;
                    tile.add_item(Item::new(item_id));

                    if let Some(id) = house_id {
                        if (1000..=1100).contains(&item_id) {
                            if let Some(house_manager) = map.house_manager_mut() {
                                let _ = house_manager.add_door_to_house(
                                    id,
                                    Door {
                                        position,
                                        id: item_id,
                                        locked: false,
                                    },
                                );
                            }
                        }
                    }
                }

                map.add_tile(tile)?;
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn area_node(z: u8, tile_props: Vec<u8>, item_ids: &[u16]) -> OtbmNode {
        OtbmNode {
            node_type: OtbmNodeType::TileArea as u8,
            props: vec![100, 0, 100, 0, z],
            children: vec![OtbmNode {
                node_type: OtbmNodeType::Tile as u8,
                props: tile_props,
                children: item_ids
                    .iter()
                    .map(|id| OtbmNode {
                        node_type: OtbmNodeType::Item as u8,
                        props: id.to_le_bytes().to_vec(),
                        children: Vec::new(),
                    })
                    .collect(),
            }],
        }
    }

    #[test]
    fn node_parser_preserves_escaped_markers_and_nested_nodes() {
        let data = [
            NODE_INIT,
            OtbmNodeType::MapData as u8,
            0xAA,
            NODE_ESCAPE,
            NODE_INIT,
            NODE_ESCAPE,
            NODE_TERM,
            NODE_ESCAPE,
            NODE_ESCAPE,
            NODE_INIT,
            OtbmNodeType::Item as u8,
            0xBB,
            NODE_TERM,
            NODE_TERM,
        ];
        let mut cursor = 0;

        let node = OtbmNode::parse(&data, &mut cursor).unwrap();

        assert_eq!(cursor, data.len());
        assert_eq!(node.props, [0xAA, NODE_INIT, NODE_TERM, NODE_ESCAPE]);
        assert_eq!(node.children.len(), 1);
        assert_eq!(node.children[0].node_type, OtbmNodeType::Item as u8);
        assert_eq!(node.children[0].props, [0xBB]);
    }

    #[test]
    fn node_parser_rejects_unterminated_and_dangling_escape_sequences() {
        let mut cursor = 0;
        let unterminated = [NODE_INIT, OtbmNodeType::MapData as u8, 0x01];
        assert!(matches!(
            OtbmNode::parse(&unterminated, &mut cursor),
            Err(OtmbError::InvalidFormat(_))
        ));

        let mut cursor = 0;
        let dangling_escape = [NODE_INIT, OtbmNodeType::MapData as u8, NODE_ESCAPE];
        assert!(matches!(
            OtbmNode::parse(&dangling_escape, &mut cursor),
            Err(OtmbError::InvalidFormat(_))
        ));
    }

    #[test]
    fn map_description_uses_description_attribute_and_skips_known_strings() {
        let mut props = vec![11, 3, 0];
        props.extend_from_slice(b"RME");
        props.extend_from_slice(&[1, 6, 0]);
        props.extend_from_slice(b"Valgor");
        props.extend_from_slice(&[13, 6, 0]);
        props.extend_from_slice(b"spawns");

        assert_eq!(OtbmLoader::parse_map_description(&props).unwrap(), "Valgor");
    }

    #[test]
    fn tile_parser_keeps_inline_ground_items_children_flags_and_coordinates() {
        let flags = (1u32 << 0) | (1u32 << 3);
        let mut tile_props = vec![2, 3, 3];
        tile_props.extend_from_slice(&flags.to_le_bytes());
        tile_props.push(9);
        tile_props.extend_from_slice(&231u16.to_le_bytes());
        let map_data = OtbmNode {
            node_type: OtbmNodeType::MapData as u8,
            props: Vec::new(),
            children: vec![area_node(7, tile_props, &[1001])],
        };
        let mut map = Map::new(256, 256, "test".to_string(), 1);

        OtbmLoader::parse_tiles(&map_data, &mut map).unwrap();

        let tile = map
            .get_tile(&Position::new(102, 103, 7))
            .expect("tile should be parsed at its absolute position");
        assert_eq!(
            tile.items.iter().map(|item| item.id).collect::<Vec<_>>(),
            [231, 1001]
        );
        assert!(tile.flags.protected);
        assert!(tile.flags.no_logout);
        assert!(!tile.flags.no_combat);
    }

    #[test]
    fn tile_parser_supports_every_tibia_floor_from_zero_through_fourteen() {
        let map_data = OtbmNode {
            node_type: OtbmNodeType::MapData as u8,
            props: Vec::new(),
            children: (0..=14)
                .map(|z| area_node(z, vec![0, 0, 9, 231, 0], &[]))
                .collect(),
        };
        let mut map = Map::new(256, 256, "test".to_string(), 1);

        OtbmLoader::parse_tiles(&map_data, &mut map).unwrap();

        assert_eq!(map.tile_count(), 15);
        assert_eq!(
            map.tiles()
                .map(|tile| tile.position.z)
                .collect::<BTreeSet<_>>(),
            BTreeSet::from_iter(0..=14)
        );
    }

    #[test]
    fn tile_parser_rejects_floor_above_fourteen() {
        let map_data = OtbmNode {
            node_type: OtbmNodeType::MapData as u8,
            props: Vec::new(),
            children: vec![area_node(15, vec![0, 0], &[])],
        };
        let mut map = Map::new(256, 256, "test".to_string(), 1);

        assert!(matches!(
            OtbmLoader::parse_tiles(&map_data, &mut map),
            Err(OtmbError::InvalidFormat(message)) if message.contains("z-level 15")
        ));
    }
}
