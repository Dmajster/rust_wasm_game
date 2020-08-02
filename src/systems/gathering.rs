use crate::{
    components::tasks::MoveToPositionTask, components::tasks::ResourceFindDropOffTask,
    components::tasks::ResourceGatherTask, components::traits::IsResourceGatherer,
    components::traits::IsResourceSource, components::ResourceType, Clock,
};
use legion::component;
use legion::{system, systems::CommandBuffer, world::SubWorld, Entity, EntityStore};

#[system(for_each)]
#[filter(!component::<MoveToPositionTask>())]
#[write_component(IsResourceSource)]
pub fn gather_task(
    #[resource] clock: &Clock,
    gather_task: &ResourceGatherTask,
    gatherer: &mut IsResourceGatherer,
    entity: &Entity,
    world: &mut SubWorld,
    command_buffer: &mut CommandBuffer,
) {
    crate::print("gathering!");

    //TODO expand this to handle destroyed entities and missing resource source component
    let mut resource_entry = world.entry_mut(gather_task.target).unwrap();

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

    //Find drop off if full
    if gatherer.resource_stored == gatherer.resource_max {
        crate::print("full of resources!");
        command_buffer.add_component(*entity, ResourceFindDropOffTask {});
    }
}

#[system]
pub fn resource_find_drop_off_task(world: &SubWorld) {

}