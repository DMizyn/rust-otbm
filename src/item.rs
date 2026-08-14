use std::collections::HashMap;

/// Represents an item in the game world
#[derive(Debug, Clone)]
pub struct Item {
    /// Unique identifier for the item type
    pub id: u16,

    /// Optional attributes for the item
    pub attributes: HashMap<ItemAttribute, AttributeValue>,
}

/// Represents different types of item attributes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemAttribute {
    // Common attributes
    Description,
    Name,
    Article,
    Plural,
    ActionId,
    UniqueId,

    // Physical attributes
    Weight,
    Attack,
    Defense,
    ExtraDefense,
    Armor,
    HitChance,
    ShootRange,

    // Appearance attributes
    SpriteId,
    AnimationId,
    CorpseId,

    // Flags
    Blocksolid,
    Blockprojectile,
    Blockpathfind,
    Protectionzone,
    Movable,
    Pickupable,
    Hangable,
    Vertical,
    Horizontal,
    Rotatable,
    Readable,
    Stackable,

    // Container attributes
    Capacity,
    ContainerType,

    // Fluid attributes
    FluidSource,
    FluidContainer,

    // Door attributes
    DoorId,

    // Teleport attributes
    TeleportDestination,

    // Magic attributes
    RuneSpell,
    Charges,

    // Light attributes
    LightLevel,
    LightColor,

    // Decay attributes
    DecayTo,
    DecayTime,

    // Special attributes
    MinimapColor,
    TradeAs,
    ShowCount,
    WrapableTo,
    UnwrapableTo,
    TopOrder,

    // Custom attributes (for extensions)
    Custom(String),
}

/// Represents the value of an item attribute
#[derive(Debug, Clone)]
pub enum AttributeValue {
    String(String),
    Integer(i32),
    Boolean(bool),
    Position { x: u16, y: u16, z: u8 },
    Color { r: u8, g: u8, b: u8 },
    List(Vec<AttributeValue>),
}

impl Item {
    /// Create a new item with the given ID
    pub fn new(id: u16) -> Self {
        Self {
            id,
            attributes: HashMap::new(),
        }
    }

    /// Set an attribute value
    pub fn set_attribute(&mut self, attr: ItemAttribute, value: AttributeValue) {
        self.attributes.insert(attr, value);
    }

    /// Get an attribute value
    pub fn get_attribute(&self, attr: &ItemAttribute) -> Option<&AttributeValue> {
        self.attributes.get(attr)
    }

    /// Remove an attribute
    pub fn remove_attribute(&mut self, attr: &ItemAttribute) -> Option<AttributeValue> {
        self.attributes.remove(attr)
    }

    /// Check if the item has a specific attribute
    pub fn has_attribute(&self, attr: &ItemAttribute) -> bool {
        self.attributes.contains_key(attr)
    }

    /// Get a string attribute
    pub fn get_string(&self, attr: &ItemAttribute) -> Option<&str> {
        match self.attributes.get(attr) {
            Some(AttributeValue::String(s)) => Some(s),
            _ => None,
        }
    }

    /// Get an integer attribute
    pub fn get_integer(&self, attr: &ItemAttribute) -> Option<i32> {
        match self.attributes.get(attr) {
            Some(AttributeValue::Integer(i)) => Some(*i),
            _ => None,
        }
    }

    /// Get a boolean attribute
    pub fn get_boolean(&self, attr: &ItemAttribute) -> Option<bool> {
        match self.attributes.get(attr) {
            Some(AttributeValue::Boolean(b)) => Some(*b),
            _ => None,
        }
    }

    /// Get a position attribute
    pub fn get_position(&self, attr: &ItemAttribute) -> Option<(u16, u16, u8)> {
        match self.attributes.get(attr) {
            Some(AttributeValue::Position { x, y, z }) => Some((*x, *y, *z)),
            _ => None,
        }
    }

    /// Get a color attribute
    pub fn get_color(&self, attr: &ItemAttribute) -> Option<(u8, u8, u8)> {
        match self.attributes.get(attr) {
            Some(AttributeValue::Color { r, g, b }) => Some((*r, *g, *b)),
            _ => None,
        }
    }

    /// Check if the item is blocking movement
    pub fn is_blocking(&self) -> bool {
        self.get_boolean(&ItemAttribute::Blocksolid)
            .unwrap_or(false)
            || self
                .get_boolean(&ItemAttribute::Blockpathfind)
                .unwrap_or(false)
    }

    /// Check if the item is stackable
    pub fn is_stackable(&self) -> bool {
        self.get_boolean(&ItemAttribute::Stackable).unwrap_or(false)
    }

    /// Check if the item is movable
    pub fn is_movable(&self) -> bool {
        self.get_boolean(&ItemAttribute::Movable).unwrap_or(true)
    }

    /// Check if the item is pickupable
    pub fn is_pickupable(&self) -> bool {
        self.get_boolean(&ItemAttribute::Pickupable)
            .unwrap_or(false)
    }

    /// Get the item weight
    pub fn weight(&self) -> u32 {
        self.get_integer(&ItemAttribute::Weight).unwrap_or(0) as u32
    }

    /// Get the item attack value
    pub fn attack(&self) -> u32 {
        self.get_integer(&ItemAttribute::Attack).unwrap_or(0) as u32
    }

    /// Get the item defense value
    pub fn defense(&self) -> u32 {
        self.get_integer(&ItemAttribute::Defense).unwrap_or(0) as u32
    }

    /// Get the item armor value
    pub fn armor(&self) -> u32 {
        self.get_integer(&ItemAttribute::Armor).unwrap_or(0) as u32
    }

    /// Get the item description
    pub fn description(&self) -> &str {
        self.get_string(&ItemAttribute::Description).unwrap_or("")
    }

    /// Get the item name
    pub fn name(&self) -> &str {
        self.get_string(&ItemAttribute::Name)
            .unwrap_or("unknown item")
    }

    /// Get the container capacity
    pub fn capacity(&self) -> u32 {
        self.get_integer(&ItemAttribute::Capacity).unwrap_or(0) as u32
    }

    /// Check if the item is a container
    pub fn is_container(&self) -> bool {
        self.has_attribute(&ItemAttribute::Capacity)
    }

    /// Check if the item is a door
    pub fn is_door(&self) -> bool {
        self.has_attribute(&ItemAttribute::DoorId)
    }

    /// Check if the item is a teleport
    pub fn is_teleport(&self) -> bool {
        self.has_attribute(&ItemAttribute::TeleportDestination)
    }

    /// Get the light level
    pub fn light_level(&self) -> u8 {
        self.get_integer(&ItemAttribute::LightLevel).unwrap_or(0) as u8
    }

    /// Get the light color
    pub fn light_color(&self) -> (u8, u8, u8) {
        self.get_color(&ItemAttribute::LightColor)
            .unwrap_or((0, 0, 0))
    }
}
