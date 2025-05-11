pub mod error;
pub mod map;
pub mod otmb;
pub mod otbm;
pub mod tile;
pub mod item;
pub mod position;
pub mod house;

pub use error::OtmbError;
pub use map::Map;
pub use otmb::OtmbLoader;
pub use otbm::OtbmLoader;
pub use position::Position;
pub use tile::Tile;
pub use item::{Item, ItemAttribute, AttributeValue};
pub use house::{House, Door, HouseManager};

/// Re-export Result type for convenience
pub type Result<T> = std::result::Result<T, error::OtmbError>;