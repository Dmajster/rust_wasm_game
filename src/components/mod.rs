pub mod tasks;
pub use tasks::*;

pub mod traits;
pub use traits::*;

use nalgebra_glm::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {
    pub food: u32,
}

pub type Position = Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResourceType {
    None,
    Food,
    Wood,
    Stone,
    Gold,
}


