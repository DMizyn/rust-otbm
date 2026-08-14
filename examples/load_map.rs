use rust_otbm::OtbmLoader;
use std::env;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the map file path from command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <path_to_otmb_file>", args[0]);
        return Ok(());
    }

    let path = Path::new(&args[1]);

    // Load the map
    println!("Loading map from {}...", path.display());
    let map = OtbmLoader::load(path)?;

    // Display map information
    println!("Map loaded successfully!");
    println!("Dimensions: {}x{}", map.width, map.height);
    println!("Description: {}", map.description);
    println!("OTMB Version: {}", map.version);
    println!("Tile count: {}", map.tile_count());

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

    Ok(())
}
