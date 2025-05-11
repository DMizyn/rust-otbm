use otmb::{HouseManager, House, Door, Position, Result};
use std::path::Path;

fn main() -> Result<()> {
    // Create a new house manager
    let mut house_manager = HouseManager::new();
    
    // Create a new house
    let mut house = House {
        id: 1,
        name: "My House".to_string(),
        entry: Position::new(100, 100, 7),
        rent: 1500,
        town_id: 1,
        size: 30,
        tiles: Default::default(),
        doors: Vec::new(),
        access_list: Vec::new(),
    };
    
    // Add some doors to the house
    house.doors.push(Door {
        position: Position::new(100, 100, 7),
        id: 1050,
        locked: false,
    });
    
    house.doors.push(Door {
        position: Position::new(105, 100, 7),
        id: 1051,
        locked: true,
    });
    
    // Add some players to the access list
    house.access_list.push("Player1".to_string());
    house.access_list.push("Player2".to_string());
    house.access_list.push("Player3".to_string());
    
    // Add the house to the manager
    house_manager.add_house(house);
    
    // Add some tiles to the house
    for x in 100..105 {
        for y in 100..105 {
            house_manager.add_tile_to_house(1, Position::new(x, y, 7))?;
        }
    }
    
    // Print house information
    println!("House count: {}", house_manager.house_count());
    
    if let Some(house) = house_manager.get_house(1) {
        println!("House: {}", house.name);
        println!("ID: {}", house.id);
        println!("Entry: ({}, {}, {})", house.entry.x, house.entry.y, house.entry.z);
        println!("Rent: {}", house.rent);
        println!("Town ID: {}", house.town_id);
        println!("Size: {}", house.size);
        println!("Tile count: {}", house.tiles.len());
        println!("Door count: {}", house.doors.len());
        
        // Print doors
        for (i, door) in house.doors.iter().enumerate() {
            println!("Door {}: ID {} at ({}, {}, {})", 
                i + 1, door.id, door.position.x, door.position.y, door.position.z);
            println!("  Locked: {}", door.locked);
        }
        
        // Print access list
        println!("Access list:");
        for player in &house.access_list {
            println!("  {}", player);
        }
    }
    
    // Save the house manager to an XML file
    let path = Path::new("example_house.xml");
    house_manager.save_to_xml(path)?;
    println!("House data saved to {}", path.display());
    
    // Load the house manager from the XML file
    let loaded_manager = HouseManager::load_from_xml(path)?;
    println!("Loaded house count: {}", loaded_manager.house_count());
    
    Ok(())
}