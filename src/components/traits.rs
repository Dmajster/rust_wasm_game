use super::ResourceType;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsResourceSource {
    pub resource_type: ResourceType,
    pub resource_left: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsResourceDropOff {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsResourceGatherer {
    pub stroke_gather_amount: u32,
    pub stroke_interval: f32, // TODO This could be stored in a tag to improve performance
    pub stroke_last_tick: u32,

    pub resource_type: ResourceType,
    pub resource_stored: u32,
    pub resource_max: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsRenderable {
    pub color: (u8, u8, u8),
    pub dimensions: (u32, u32),
}
