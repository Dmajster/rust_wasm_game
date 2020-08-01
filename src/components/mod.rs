use legion::entity::Entity;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {
    pub food: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GatherTask {
    pub target: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FindResourceDropOffTask {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResourceDropOffTask {
    pub target: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsResourceDropOff {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsResourceGatherer {
    pub stroke_gathered_amount: u32,
    pub stroke_interval: f32, // TODO This could be stored in a tag to improve performance
    pub stroke_last_tick: u32,

    pub resource_type: ResourceType,
    pub resource_stored: u32,
    pub resource_max_stored: u32, // TODO This could be stored in a tag to improve performance
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResourceType {
    None,
    Food,
    Wood,
    Stone,
    Gold,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsResource {
    pub resource_type: ResourceType,
    pub resource_stored_amount: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MoveTowardsTargetTask {
    pub target: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsRenderable {
    pub color: (u8, u8, u8),
    pub dimensions: (u32, u32),
}