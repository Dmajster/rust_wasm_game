use legion::Entity;
use nalgebra_glm::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MoveToEntityTask {
    pub entity: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MoveToPositionTask {
    pub position: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResourceGatherTask {
    pub target: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResourceDropOffTask {
    pub target: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResourceFindDropOffTask {}
