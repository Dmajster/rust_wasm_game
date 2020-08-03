use crate::MoveToEntityTask;
use crate::Position;
use crate::ResourceDropOffTask;
use crate::{
    components::IsResourceGatherer,
    components::IsResourceSource,
    components::MoveToPositionTask,
    components::ResourceFindDropOffTask,
    components::ResourceGatherTask,
    components::{IsResourceDropOff, ResourceType},
    Clock,
};
use legion::component;
use legion::{system, systems::CommandBuffer, world::SubWorld, Entity, EntityStore, IntoQuery};

#[system(for_each)]
#[filter(!component::<MoveToPositionTask>() & component::<ResourceGatherTask>())]
#[write_component(IsResourceSource)]
pub fn resource_gather_task(
    #[resource] clock: &Clock,
    gather_task: &ResourceGatherTask,
    gatherer: &mut IsResourceGatherer,
    entity: &Entity,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
) {
    let resource_entry = world.entry_mut(gather_task.target);

    //If the resource entity was removed remove this task.
    if resource_entry.is_none() {
        command_buffer.remove_component::<ResourceGatherTask>(*entity);
        command_buffer.add_component(*entity, ResourceFindDropOffTask {});
        return;
    }

    let mut resource_entry = resource_entry.unwrap();

    let resource_source = resource_entry
        .get_component_mut::<IsResourceSource>()
        .unwrap();

    //Drop off resources if trying to mine a resource while we have a different one in storage
    if gatherer.resource_type != ResourceType::None
        && gatherer.resource_type != resource_source.resource_type
    {
        command_buffer.add_component(*entity, ResourceFindDropOffTask {});
        return;
    }

    //Ensure you can't gather more than is left at the resource source or is available in gatherer's storage
    let gathered_resource_amount = resource_source
        .resource_left
        .min(gatherer.stroke_gather_amount)
        .min(gatherer.resource_max - gatherer.resource_stored);

    //Gather the resources
    resource_source.resource_left -= gathered_resource_amount;
    gatherer.resource_stored += gathered_resource_amount;
    gatherer.resource_type = resource_source.resource_type;
    crate::print(format!(
        "gatherer: {:#?}\nsource: {:#?}",
        gatherer, resource_source
    ));

    //Find drop off if full
    if gatherer.resource_stored == gatherer.resource_max {
        crate::print("full of resources!".to_owned());
        command_buffer.add_component(*entity, ResourceFindDropOffTask {});
    }

    if resource_source.resource_left == 0 {
        command_buffer.remove(gather_task.target);
    }
}

#[system]
#[read_component(Position)]
#[read_component(ResourceFindDropOffTask)]
#[read_component(IsResourceDropOff)]
#[filter(component::<ResourceFindDropOffTask>())]
pub fn resource_find_drop_off_task(world: &SubWorld, command_buffer: &mut CommandBuffer) {
    let mut gatherers_query = <(Entity, &Position, &ResourceFindDropOffTask)>::query();
    let mut drop_offs_query = <(Entity, &Position, &IsResourceDropOff)>::query();

    for (gatherer, gatherer_position, _) in gatherers_query.iter(world) {
        crate::print("searching for drop off!".to_owned());

        let mut closest_entity: Option<&Entity> = None;
        let mut closest_distance = f32::MAX;

        //Iterate over possible drop off entities and find the closest one
        for (drop_off, drop_off_position, _) in drop_offs_query.iter(world) {
            let current_distance = nalgebra_glm::distance(gatherer_position, drop_off_position);

            if current_distance < closest_distance {
                closest_distance = current_distance;
                closest_entity = Some(drop_off);
            }
        }

        //If we found any valid drop off entity set is as the drop off target
        if let Some(target_entity) = closest_entity {
            crate::print("set drop off!".to_owned());

            command_buffer.add_component(
                *gatherer,
                MoveToEntityTask {
                    entity: *target_entity,
                },
            );

            command_buffer.add_component(
                *gatherer,
                ResourceDropOffTask {
                    target: *target_entity,
                },
            );
            command_buffer.remove_component::<ResourceFindDropOffTask>(*gatherer);
        }
    }
}

#[system(for_each)]
#[filter(!component::<MoveToPositionTask>() & component::<ResourceDropOffTask>())]
pub fn resource_drop_off_task(
    #[resource] clock: &Clock,
    gather_task: Option<&ResourceGatherTask>,
    gatherer: &mut IsResourceGatherer,
    entity: &Entity,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
) {
    //TODO increment players resources when that's a thing...
    crate::print("dropped off resources".to_owned());

    gatherer.resource_stored = 0;
    gatherer.resource_type = ResourceType::None;

    command_buffer.remove_component::<ResourceDropOffTask>(*entity);

    if let Some(gather_task) = gather_task {
        command_buffer.add_component(
            *entity,
            MoveToEntityTask {
                entity: gather_task.target,
            },
        );
    }
}
