use rust_otbm::{AttributeValue, Item, ItemAttribute};

fn main() {
    // Create a new item
    let mut item = Item::new(1234);

    // Set various attributes
    item.set_attribute(
        ItemAttribute::Name,
        AttributeValue::String("Magic Sword".to_string()),
    );
    item.set_attribute(
        ItemAttribute::Description,
        AttributeValue::String("A powerful magical sword".to_string()),
    );
    item.set_attribute(ItemAttribute::Weight, AttributeValue::Integer(3500));
    item.set_attribute(ItemAttribute::Attack, AttributeValue::Integer(45));
    item.set_attribute(ItemAttribute::Defense, AttributeValue::Integer(20));
    item.set_attribute(ItemAttribute::Armor, AttributeValue::Integer(10));

    // Set boolean flags
    item.set_attribute(ItemAttribute::Movable, AttributeValue::Boolean(true));
    item.set_attribute(ItemAttribute::Pickupable, AttributeValue::Boolean(true));
    item.set_attribute(ItemAttribute::Stackable, AttributeValue::Boolean(false));
    item.set_attribute(ItemAttribute::Blocksolid, AttributeValue::Boolean(false));

    // Set light properties
    item.set_attribute(ItemAttribute::LightLevel, AttributeValue::Integer(5));
    item.set_attribute(
        ItemAttribute::LightColor,
        AttributeValue::Color {
            r: 255,
            g: 180,
            b: 100,
        },
    );

    // Set a teleport destination
    item.set_attribute(
        ItemAttribute::TeleportDestination,
        AttributeValue::Position {
            x: 1000,
            y: 1000,
            z: 7,
        },
    );

    // Set a custom attribute
    item.set_attribute(
        ItemAttribute::Custom("enchantment_level".to_string()),
        AttributeValue::Integer(3),
    );

    // Print item information
    println!("Item ID: {}", item.id);
    println!("Name: {}", item.name());
    println!("Description: {}", item.description());
    println!("Weight: {} oz", item.weight());
    println!("Attack: {}", item.attack());
    println!("Defense: {}", item.defense());
    println!("Armor: {}", item.armor());

    // Check flags
    println!("Is movable: {}", item.is_movable());
    println!("Is pickupable: {}", item.is_pickupable());
    println!("Is stackable: {}", item.is_stackable());
    println!("Is blocking: {}", item.is_blocking());

    // Check light properties
    println!("Light level: {}", item.light_level());
    let (r, g, b) = item.light_color();
    println!("Light color: RGB({}, {}, {})", r, g, b);

    // Check teleport destination
    if let Some((x, y, z)) = item.get_position(&ItemAttribute::TeleportDestination) {
        println!("Teleport destination: ({}, {}, {})", x, y, z);
    }

    // Check custom attribute
    if let Some(AttributeValue::Integer(level)) =
        item.get_attribute(&ItemAttribute::Custom("enchantment_level".to_string()))
    {
        println!("Enchantment level: {}", level);
    }
}
