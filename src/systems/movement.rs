use crate::{
    components::{MoveToEntityTask, MoveToPositionTask, Position},
    Clock,
};

use legion::{system, systems::CommandBuffer, world::SubWorld, Entity, EntityStore, World};
use nalgebra_glm::Vec2;

#[system(for_each)]
#[read_component(Position)]
pub fn move_to_entity_task(
    move_to_entity_task: &MoveToEntityTask,
    entity: &Entity,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
) {
    // Ensure target entity didn't get deleted
    if let Some(mut entity_entry) = world.entry_mut(move_to_entity_task.entity) {
        let target_position = entity_entry.get_component::<Position>().unwrap().clone();

        match entity_entry.get_component_mut::<MoveToPositionTask>() {
            Ok(move_to_position_task) => {
                move_to_position_task.position = target_position;
            }
            Err(_) => {
                command_buffer.add_component(
                    *entity,
                    MoveToPositionTask {
                        position: target_position,
                    },
                );
            }
        }
    }
}

#[system(for_each)]
pub fn move_to_position_task(
    #[resource] clock: &Clock,
    move_to_position_task: &MoveToPositionTask,
    position: &mut Position,
    entity: &Entity,
    command_buffer: &mut CommandBuffer,
) {
    if nalgebra_glm::distance(&position, &move_to_position_task.position) < 0.1 {
        crate::print("reached destination!".to_owned());
        command_buffer.remove_component::<MoveToEntityTask>(*entity);
        command_buffer.remove_component::<MoveToPositionTask>(*entity);
        return;
    }

    let mut direction: Vec2 = move_to_position_task.position.xy() - position.xy();
    direction.normalize_mut();
    direction *= clock.time_delta * 64.0;

    position.x += direction.x;
    position.y += direction.y;
}
