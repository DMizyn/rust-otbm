use rust_otbm::{AttributeValue, Item, ItemAttribute};

#[test]
fn test_basic_attributes() {
    let mut item = Item::new(1234);

    // Set basic attributes
    item.set_attribute(
        ItemAttribute::Name,
        AttributeValue::String("Test Item".to_string()),
    );
    item.set_attribute(
        ItemAttribute::Description,
        AttributeValue::String("A test item".to_string()),
    );
    item.set_attribute(ItemAttribute::Weight, AttributeValue::Integer(1000));

    // Check attribute values
    assert_eq!(item.name(), "Test Item");
    assert_eq!(item.description(), "A test item");
    assert_eq!(item.weight(), 1000);

    // Check has_attribute
    assert!(item.has_attribute(&ItemAttribute::Name));
    assert!(item.has_attribute(&ItemAttribute::Description));
    assert!(item.has_attribute(&ItemAttribute::Weight));
    assert!(!item.has_attribute(&ItemAttribute::Attack));

    // Remove an attribute
    item.remove_attribute(&ItemAttribute::Name);
    assert!(!item.has_attribute(&ItemAttribute::Name));
    assert_eq!(item.name(), "unknown item"); // Default value
}

#[test]
fn test_boolean_attributes() {
    let mut item = Item::new(1234);

    // Initially not blocking
    assert!(!item.is_blocking());

    // Set blocking attributes
    item.set_attribute(ItemAttribute::Blocksolid, AttributeValue::Boolean(true));
    assert!(item.is_blocking());

    // Set other boolean attributes
    item.set_attribute(ItemAttribute::Movable, AttributeValue::Boolean(false));
    item.set_attribute(ItemAttribute::Pickupable, AttributeValue::Boolean(true));
    item.set_attribute(ItemAttribute::Stackable, AttributeValue::Boolean(true));

    // Check values
    assert!(!item.is_movable());
    assert!(item.is_pickupable());
    assert!(item.is_stackable());
}

#[test]
fn test_position_attribute() {
    let mut item = Item::new(1234);

    // Set teleport destination
    item.set_attribute(
        ItemAttribute::TeleportDestination,
        AttributeValue::Position {
            x: 100,
            y: 200,
            z: 7,
        },
    );

    // Check if it's a teleport
    assert!(item.has_attribute(&ItemAttribute::TeleportDestination));
    assert!(item.is_teleport());

    // Check position value
    if let Some((x, y, z)) = item.get_position(&ItemAttribute::TeleportDestination) {
        assert_eq!(x, 100);
        assert_eq!(y, 200);
        assert_eq!(z, 7);
    } else {
        panic!("Expected teleport destination position");
    }
}

#[test]
fn test_color_attribute() {
    let mut item = Item::new(1234);

    // Set light color
    item.set_attribute(
        ItemAttribute::LightColor,
        AttributeValue::Color {
            r: 255,
            g: 128,
            b: 64,
        },
    );

    // Check color value
    let (r, g, b) = item.light_color();
    assert_eq!(r, 255);
    assert_eq!(g, 128);
    assert_eq!(b, 64);
}

#[test]
fn test_custom_attributes() {
    let mut item = Item::new(1234);

    // Set custom attributes
    item.set_attribute(
        ItemAttribute::Custom("enchantment".to_string()),
        AttributeValue::Integer(3),
    );

    item.set_attribute(
        ItemAttribute::Custom("creator".to_string()),
        AttributeValue::String("Wizard".to_string()),
    );

    // Check custom attribute values
    if let Some(AttributeValue::Integer(level)) =
        item.get_attribute(&ItemAttribute::Custom("enchantment".to_string()))
    {
        assert_eq!(*level, 3);
    } else {
        panic!("Expected enchantment attribute");
    }

    if let Some(AttributeValue::String(creator)) =
        item.get_attribute(&ItemAttribute::Custom("creator".to_string()))
    {
        assert_eq!(creator, "Wizard");
    } else {
        panic!("Expected creator attribute");
    }
}
