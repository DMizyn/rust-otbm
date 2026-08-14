# OTMB Map Loader

A Rust library for loading and using OTMB map files from The Forgotten Server project.

## Overview

This library provides functionality to load, parse, and use OTMB map files in your Rust applications. OTMB is a file format used by The Forgotten Server project and map editors like Remere's Map Editor to store game maps for MMO servers.

## Features

- Load and save OTMB map files
- Support for Remere's Map Editor OTBM files
- Access map tiles, items, and properties
- Query map data for game logic (e.g., pathfinding, collision detection)
- Support for multiple OTMB versions (1, 2, and 3)
- Comprehensive item attribute system matching The Forgotten Server
- Full support for house data including doors and access lists
- Support for XML-based house and spawn data

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
otmb = "0.1.0"
```

### Example: Loading a Map

```rust
use otmb::OtmbLoader;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load a map file
    let map = OtmbLoader::load(Path::new("my_map.otmb"))?;
    
    // Display map information
    println!("Map dimensions: {}x{}", map.width, map.height);
    println!("Total tiles: {}", map.tile_count());
    
    // Check if a position is walkable
    let pos = otmb::Position::new(100, 100, 7);
    if map.is_walkable(&pos) {
        println!("Position is walkable!");
    } else {
        println!("Position is not walkable!");
    }
    
    Ok(())
}
```

### Example: Loading a Remere's Map Editor File

```rust
use otmb::OtbmLoader;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load a Remere's Map Editor file
    let map = OtbmLoader::load(Path::new("my_map.otbm"))?;
    
    // Display map information
    println!("Map dimensions: {}x{}", map.width, map.height);
    println!("Description: {}", map.description);
    println!("Total tiles: {}", map.tile_count());
    
    // Access house information
    if let Some(house_manager) = map.house_manager() {
        println!("Houses: {}", house_manager.house_count());
        
        // Get a specific house
        if let Some(house) = house_manager.get_house(1) {
            println!("House name: {}", house.name);
            println!("House rent: {}", house.rent);
            println!("House doors: {}", house.doors.len());
            println!("Is guild hall: {}", house.guild_hall);
        }
    }
    
    Ok(())
}
```

### Example: Working with Houses

```rust
use otmb::{HouseManager, House, Door, Position};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load houses from XML
    let mut house_manager = HouseManager::load_from_xml("houses.xml")?;
    
    // Get a house
    if let Some(house) = house_manager.get_house(1) {
        println!("House: {}", house.name);
        println!("Entry: ({}, {}, {})", house.entry.x, house.entry.y, house.entry.z);
        println!("Rent: {}", house.rent);
        
        // Check doors
        for door in &house.doors {
            println!("Door at ({}, {}, {})", door.position.x, door.position.y, door.position.z);
            println!("Locked: {}", door.locked);
        }
        
        // Check access list
        for player in &house.access_list {
            println!("Access: {}", player);
        }
    }
    
    // Add a player to access list
    house_manager.add_player_to_access_list(1, "NewPlayer".to_string())?;
    
    // Save houses to XML
    house_manager.save_to_xml("updated_houses.xml")?;
    
    Ok(())
}
```

### Example: Working with Items and Attributes

```rust
use otmb::{Item, ItemAttribute, AttributeValue};

// Create a new item
let mut item = Item::new(1234);

// Set various attributes
item.set_attribute(ItemAttribute::Name, AttributeValue::String("Magic Sword".to_string()));
item.set_attribute(ItemAttribute::Attack, AttributeValue::Integer(45));
item.set_attribute(ItemAttribute::Defense, AttributeValue::Integer(20));
item.set_attribute(ItemAttribute::Movable, AttributeValue::Boolean(true));

// Set light properties
item.set_attribute(ItemAttribute::LightLevel, AttributeValue::Integer(5));
item.set_attribute(ItemAttribute::LightColor, AttributeValue::Color { r: 255, g: 180, b: 100 });

// Check item properties
println!("Item name: {}", item.name());
println!("Attack: {}", item.attack());
println!("Is movable: {}", item.is_movable());
```

## Supported Item Attributes

The library supports all item attributes used in The Forgotten Server:

### Basic Attributes
- Name, Description, Article, Plural, ActionId, UniqueId
- Weight, Attack, Defense, ExtraDefense, Armor, HitChance, ShootRange

### Appearance Attributes
- SpriteId, AnimationId, CorpseId

### Flags
- Blocksolid, Blockprojectile, Blockpathfind
- Protectionzone, Movable, Pickupable
- Hangable, Vertical, Horizontal, Rotatable
- Readable, Stackable

### Container Attributes
- Capacity, ContainerType

### Fluid Attributes
- FluidSource, FluidContainer

### Door Attributes
- DoorId

### Teleport Attributes
- TeleportDestination

### Magic Attributes
- RuneSpell, Charges

### Light Attributes
- LightLevel, LightColor

### Decay Attributes
- DecayTo, DecayTime

### Special Attributes
- MinimapColor, TradeAs, ShowCount
- WrapableTo, UnwrapableTo, TopOrder

### Custom Attributes
- Support for custom named attributes

The OTBM loader preserves serialized action ids, unique ids, text, charges,
teleport destinations, depot ids, house-door ids and the remaining standard
item attributes on both inline ground items and child item nodes. Game servers
can therefore enforce door, key, quest and tool behavior from map metadata.

## House System

The library includes a comprehensive house system:

- **House Properties**: ID, name, entry position, rent, size, town ID
- **Guild Halls**: Support for guild halls with special properties
- **House Tiles**: Track which tiles belong to which house
- **Doors**: Support for house doors with positions and lock status
- **Access Lists**: Track which players have access to a house
- **XML Support**: Load and save house data from/to XML files

## Running the Examples

```
cargo run --example load_map path/to/your/map.otmb
cargo run --example load_remere_map path/to/your/remere_map.otbm
cargo run --example house_manager
cargo run --example item_attributes
```

## File Format Support

### OTMB Format
The OTMB format is a binary format used to store map data. It includes:

- Map dimensions
- Map description
- Tiles with positions
- Items on tiles
- Tile and item properties

This library supports OTMB versions 1, 2, and 3.

### OTBM Format (Remere's Map Editor)
The OTBM format is used by Remere's Map Editor and has a slightly different structure:

- Starts with 4 zero bytes
- Contains map dimensions and description
- Organizes tiles and items in a hierarchical node structure
- Includes additional metadata like spawn points and house information

### XML Support
Remere's Map Editor uses XML files for additional data:

- **House XML**: Contains house definitions with doors and access lists
- **Spawn XML**: Contains spawn areas with monster types and spawn times

## License

This project is licensed under the MIT License - see the LICENSE file for details.
