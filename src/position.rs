/// Represents a 3D position in the game world
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u16,
    pub y: u16,
    pub z: u8,
}

impl Position {
    /// Create a new position
    pub fn new(x: u16, y: u16, z: u8) -> Self {
        Self { x, y, z }
    }

    /// Calculate the distance between two positions (ignoring z-axis)
    pub fn distance_to(&self, other: &Position) -> f32 {
        let dx = self.x as f32 - other.x as f32;
        let dy = self.y as f32 - other.y as f32;
        (dx * dx + dy * dy).sqrt()
    }

    /// Check if this position is adjacent to another position
    pub fn is_adjacent_to(&self, other: &Position) -> bool {
        if self.z != other.z {
            return false;
        }

        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);

        dx <= 1 && dy <= 1 && (dx + dy > 0)
    }
}
