pub mod error;
pub mod house;
pub mod item;
pub mod map;
pub mod otbm;
pub mod position;
pub mod tile;

pub use error::OtmbError;
pub use house::{Door, House, HouseManager};
pub use item::{AttributeValue, Item, ItemAttribute};
pub use map::Map;
pub use otbm::OtbmLoader;
pub use position::Position;
pub use tile::Tile;

/// Re-export Result type for convenience
pub type Result<T> = std::result::Result<T, error::OtmbError>;
