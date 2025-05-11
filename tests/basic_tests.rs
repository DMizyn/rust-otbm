use otmb::{Map, Position, Tile, Item, OtmbError};

#[test]
fn test_position() {
    let pos1 = Position::new(10, 20, 7);
    let pos2 = Position::new(11, 20, 7);
    let pos3 = Position::new(10, 20, 8);
    
    assert_eq!(pos1.x, 10);
    assert_eq!(pos1.y, 20);
    assert_eq!(pos1.z, 7);
    
    assert!(pos1.is_adjacent_to(&pos2));
    assert!(!pos1.is_adjacent_to(&pos3)); // Different z level
    
    let distance = pos1.distance_to(&pos2);
    assert!((distance - 1.0).abs() < 0.001);
}

#[test]
fn test_map_bounds() {
    let mut map = Map::new(100, 100, "Test Map".to_string(), 1);
    
    // Valid position
    let valid_pos = Position::new(50, 50, 7);
    let valid_tile = Tile::new(valid_pos);
    assert!(map.add_tile(valid_tile).is_ok());
    
    // Invalid position (outside bounds)
    let invalid_pos = Position::new(200, 50, 7);
    let invalid_tile = Tile::new(invalid_pos);
    assert!(matches!(map.add_tile(invalid_tile), Err(OtmbError::InvalidDimensions(_))));
}

#[test]
fn test_tile_items() {
    let pos = Position::new(10, 10, 7);
    let mut tile = Tile::new(pos);
    
    // Initially walkable
    assert!(tile.is_walkable());
    
    // Add a blocking item (ID divisible by 5 is blocking in our implementation)
    let blocking_item = Item::new(5);
    tile.add_item(blocking_item);
    
    // Should now be blocking
    assert!(!tile.is_walkable());
    
    // Add a non-blocking item
    let non_blocking_item = Item::new(6);
    tile.add_item(non_blocking_item);
    
    // Should still be blocking
    assert!(!tile.is_walkable());
    
    // Check ground item
    assert_eq!(tile.ground_item().unwrap().id, 5);
}