pub mod prelude {
    pub use super::Block;
    pub use super::BlockType;
}

pub struct Block {
    pub block_type: BlockType,
    pub pos:        (f32, f32),
}

pub enum BlockType {
    Empty,
    Wall,
    Enemy,
}

impl BlockType {
    pub fn z(&self) -> f32 {
        match self {
            Self::Empty => 0.0,
            Self::Wall => 2.0,
            Self::Enemy => 0.8,
        }
    }
}

impl From<char> for BlockType {
    fn from(chr: char) -> Self {
        match chr {
            '-' => Self::Empty,
            '#' => Self::Wall,
            'X' => Self::Enemy,
            _ => Self::default(),
        }
    }
}

impl Default for BlockType {
    fn default() -> Self {
        Self::Empty
    }
}
