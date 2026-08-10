use rust_otbm::{HouseManager, OtbmLoader, Position, Result};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[test]
fn test_load_remere_map() -> Result<()> {
    // Path to the test OTBM file
    let test_file_path = Path::new("tests/1/test.otbm");

    // Ensure the test file exists
    assert!(
        test_file_path.exists(),
        "Test file does not exist: {:?}",
        test_file_path
    );

    // Try to load the map using the OtbmLoader
    println!("Loading map from {:?}", test_file_path);
    let map = OtbmLoader::load(test_file_path)?;

    // Print map information
    println!("Map loaded successfully!");
    println!("Dimensions: {}x{}", map.width, map.height);

    // Check that the map dimensions are 2048x2048
    assert_eq!(map.width, 2048, "Map width should be 2048");
    assert_eq!(map.height, 2048, "Map height should be 2048");

    println!("Description: {}", map.description);
    println!("OTBM Version: {}", map.version);
    println!("Tile count: {}", map.tile_count());
    assert_eq!(
        map.tile_count(),
        2116,
        "All tiles from the fixture must be parsed"
    );

    let z_levels: BTreeSet<u8> = map.tiles().map(|tile| tile.position.z).collect();
    assert_eq!(z_levels, BTreeSet::from([7]), "Unexpected map floors");

    // Count walkable and non-walkable tiles
    let mut walkable = 0;
    let mut non_walkable = 0;

    for tile in map.tiles() {
        if tile.is_walkable() {
            walkable += 1;
        } else {
            non_walkable += 1;
        }
    }

    println!("Walkable tiles: {}", walkable);
    println!("Non-walkable tiles: {}", non_walkable);

    // Check a known tile and item from the fixture.
    let backpack_pos = Position::new(9, 17, 7);
    let tile = map
        .get_tile(&backpack_pos)
        .expect("Known fixture tile must exist");
    assert_eq!(tile.items.first().map(|item| item.id), Some(4526));

    // Check for house information in the XML file
    let house_file_path = Path::new("tests/1/test-house.xml");
    if house_file_path.exists() {
        // Load houses directly from XML
        let house_manager = HouseManager::load_from_xml(house_file_path)?;

        println!("House XML file found");

        // Check if there are two houses
        let house_count = house_manager.house_count();
        println!("Number of houses found: {}", house_count);
        assert_eq!(house_count, 2, "Expected 2 houses in the house XML file");

        // Get the first house
        let house = house_manager.get_house(1).expect("House should exist");

        // Print house details
        println!("House details:");
        println!("  ID: {}", house.id);
        println!("  Name: {}", house.name);
        println!(
            "  Entry: ({}, {}, {})",
            house.entry.x, house.entry.y, house.entry.z
        );
        println!("  Rent: {}", house.rent);
        println!("  Town ID: {}", house.town_id);
        println!("  Size: {}", house.size);

        // Check house properties
        assert_eq!(house.name, "Unnamed House #1", "House name should match");
        assert_eq!(house.id, 1, "House ID should be 1");
        assert_eq!(house.rent, 1000, "House rent should be 1000");
        assert_eq!(house.town_id, 1, "House town ID should be 1");
        assert_eq!(house.size, 25, "House size should be 25");

        // Get the second house (guild hall)
        let guild_house = house_manager
            .get_house(2)
            .expect("Guild house should exist");
        println!("\nGuild house details:");
        println!("  ID: {}", guild_house.id);
        println!("  Name: {}", guild_house.name);
        println!("  Rent: {}", guild_house.rent);
        println!("  Size: {}", guild_house.size);

        // Check guild house properties
        assert_eq!(
            guild_house.name, "Wolfwatch Tower",
            "Guild house name should match"
        );
        assert_eq!(guild_house.rent, 5000, "Guild house rent should be 5000");

        // Check if the houses are also loaded in the map
        if let Some(map_house_manager) = map.house_manager() {
            let map_house_count = map_house_manager.house_count();
            println!("Houses loaded in map: {}", map_house_count);

            if map_house_count > 0 {
                if let Some(map_house) = map_house_manager.get_house(1) {
                    println!("House from map: {}", map_house.name);

                    // Check doors
                    println!("House doors: {}", map_house.doors.len());
                    for (i, door) in map_house.doors.iter().enumerate() {
                        println!(
                            "  Door {}: ID {} at ({}, {}, {})",
                            i + 1,
                            door.id,
                            door.position.x,
                            door.position.y,
                            door.position.z
                        );
                    }

                    // Check access list
                    println!("Access list entries: {}", map_house.access_list.len());
                    for (i, player) in map_house.access_list.iter().enumerate() {
                        println!("  Player {}: {}", i + 1, player);
                    }
                }
            }
        }
    } else {
        println!("House XML file not found");
    }

    // Check for spawn information in the XML file
    let spawn_file_path = Path::new("tests/1/test-spawn.xml");
    if spawn_file_path.exists() {
        let mut spawn_file = File::open(spawn_file_path)?;
        let mut spawn_content = String::new();
        spawn_file.read_to_string(&mut spawn_content)?;

        println!("Spawn XML file found");

        // Check for "bug" creatures
        let bug_count = spawn_content.matches("name=\"bug\"").count();
        println!("Number of 'bug' creatures found: {}", bug_count);

        // Count spawn areas
        let spawn_areas = spawn_content.matches("<spawn ").count();
        println!("Number of spawn areas: {}", spawn_areas);

        // The file contains 6 bug instances across 3 spawn areas
        assert_eq!(
            bug_count, 6,
            "Expected 6 'bug' creatures in the spawn XML file"
        );
        assert_eq!(
            spawn_areas, 3,
            "Expected 3 spawn areas in the spawn XML file"
        );
    } else {
        println!("Spawn XML file not found");
    }

    // Basic validation
    assert!(map.width > 0, "Map width should be positive");
    assert!(map.height > 0, "Map height should be positive");
    assert!(map.tile_count() > 0, "Map should have at least one tile");

    Ok(())
}
