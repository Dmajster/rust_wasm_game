mod components;
use components::*;

use legion::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub struct Game {
    world: World,
    resources: Resources,
    update_schedule: Schedule,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CanvasRenderer {
    context: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let move_towards_target_system = SystemBuilder::new("move_towards_target_system")
            .with_query(<(Read<MoveTowardsTargetTask>, Write<Position>)>::query())
            .read_component::<Position>()
            .build(move |command_buffer, world, (), query| unsafe {
                for (entity, (move_target, mut position)) in query.iter_entities_unchecked(world) {
                    let target_position =
                        world.get_component::<Position>(move_target.target).unwrap();

                    let target_direction = (target_position.x - position.x).max(-1.0).min(1.0);

                    position.x += target_direction * 0.16;

                    if target_direction.abs() <= 0.01 {
                        command_buffer.remove_component::<MoveTowardsTargetTask>(entity);
                    }
                }
            });

        let gather_task_system = SystemBuilder::new("gather_task_system")
            .with_query(
                <(Read<GatherTask>, Write<IsResourceGatherer>)>::query().filter(
                    !component::<MoveTowardsTargetTask>()
                        & !component::<ResourceDropOffTask>()
                        & !component::<FindResourceDropOffTask>(),
                ),
            )
            .write_component::<IsResource>()
            .build(move |command_buffer, world, (), query| {
                let (mut query_world, mut rest_of_world) = SubWorld::split_for_query(world, query);

                for (entity, (task, mut gatherer)) in query.iter_entities_mut(&mut query_world) {
                    let mut gatherable =
                        match rest_of_world.get_component_mut::<IsResource>(task.target) {
                            Some(gatherable) => gatherable,
                            None => {
                                command_buffer.remove_component::<GatherTask>(entity);
                                continue;
                            }
                        };

                    if gatherable.resource_stored_amount > 0 {
                        // This is the capacity the villager could potentialy take, ensures villagers don't overfill their storage
                        let max_gather_amount = (gatherer.resource_max_stored
                            - gatherer.resource_stored)
                            .min(gatherer.stroke_gathered_amount);

                        // This is the capacity they will actually take, ensures they can't take more resources than are left in the gatherable
                        let gathered_amount =
                            gatherable.resource_stored_amount.min(max_gather_amount);

                        gatherable.resource_stored_amount -= gathered_amount;
                        gatherer.resource_stored += gathered_amount;
                        gatherer.resource_type = gatherable.resource_type;
                    }

                    if gatherer.resource_stored == gatherer.resource_max_stored {
                        command_buffer.add_component::<FindResourceDropOffTask>(
                            entity,
                            FindResourceDropOffTask {},
                        );
                    }

                    if gatherable.resource_stored_amount == 0 {
                        command_buffer.remove_component::<IsResource>(task.target);
                        command_buffer.remove_component::<GatherTask>(entity);
                    }
                }
            });

        let find_drop_off_task_system = SystemBuilder::new("find_drop_off_task_system")
            .with_query(<(Read<FindResourceDropOffTask>, Read<Position>)>::query())
            .read_component::<Position>()
            .read_component::<IsResourceDropOff>()
            .build(move |command_buffer, world, (), query| {
                let drop_off_query = <(Read<IsResourceDropOff>, Read<Position>)>::query();

                for (entity, (task, position)) in query.iter_entities(world) {
                    let mut min_distance: f64 = f64::MAX;
                    let mut min_entity: Option<Entity> = None;

                    for (drop_off_entity, (_, drop_off_position)) in
                        drop_off_query.iter_entities(world)
                    {
                        let distance = (position.x - drop_off_position.x).abs();

                        if distance < min_distance {
                            min_distance = distance;
                            min_entity = Some(drop_off_entity);
                        }
                    }

                    if min_entity.is_some() {
                        command_buffer.add_component(
                            entity,
                            ResourceDropOffTask {
                                target: min_entity.unwrap(),
                            },
                        );
                        command_buffer.add_component(
                            entity,
                            MoveTowardsTargetTask {
                                target: min_entity.unwrap(),
                            },
                        );
                    }

                    command_buffer.remove_component::<FindResourceDropOffTask>(entity);
                }
            });

        let drop_off_task_system = SystemBuilder::new("drop_off_task_system")
            .with_query(
                <(
                    Read<GatherTask>,
                    Read<ResourceDropOffTask>,
                    Write<IsResourceGatherer>,
                )>::query()
                .filter(!component::<MoveTowardsTargetTask>()),
            )
            .build(move |command_buffer, world, (), query| {
                for (entity, (gather_task, drop_off_task, mut gatherer)) in
                    query.iter_entities_mut(world)
                {
                    gatherer.resource_stored = 0;
                    gatherer.resource_type = ResourceType::None;

                    command_buffer.remove_component::<ResourceDropOffTask>(entity);
                    command_buffer.add_component(
                        entity,
                        MoveTowardsTargetTask {
                            target: gather_task.target,
                        },
                    )
                }
            });

        let draw_system = SystemBuilder::new("drop_off_task_system")
            .with_query(<(Read<Position>, Tagged<IsRenderable>)>::query())
            .build(move |command_buffer, world, (), query| {
                let document = web_sys::window().unwrap().document().unwrap();
                let canvas = document.get_element_by_id("canvas").unwrap();
                let canvas: web_sys::HtmlCanvasElement = canvas
                    .dyn_into::<web_sys::HtmlCanvasElement>()
                    .map_err(|_| ())
                    .unwrap();

                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .unwrap();

                let context: CanvasRenderingContext2d = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .unwrap();

                context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

                for (position, renderable) in query.iter(world) {
                    context.begin_path();

                    context.set_fill_style(
                        &format!(
                            "rgb({},{},{})",
                            renderable.color.0, renderable.color.1, renderable.color.2
                        )
                        .into(),
                    );

                    context.rect(
                        position.x as f64,
                        position.y as f64,
                        renderable.dimensions.0 as f64,
                        renderable.dimensions.1 as f64,
                    );

                    context.fill();
                }
            });

        let update_schedule = Schedule::builder()
            .add_system(move_towards_target_system)
            .add_system(gather_task_system)
            .add_system(find_drop_off_task_system)
            .add_system(drop_off_task_system)
            .add_system(draw_system)
            .build();

        Self {
            world: World::new(),
            resources: Resources::default(),
            update_schedule,
        }
    }

    pub fn start(&mut self) {
        let town_center_sprite = IsRenderable {
            color: (255, 0, 0),
            dimensions: (32, 32),
        };

        let berry_bush_sprite = IsRenderable {
            color: (0, 255, 0),
            dimensions: (8, 8),
        };

        let villager_sprite = IsRenderable {
            color: (255, 0, 0),
            dimensions: (4, 8),
        };

        let town_center = self.world.insert(
            (town_center_sprite,),
            vec![(Position { x: 64.0, y: 64.0 }, IsResourceDropOff {})],
        );

        let berry_bush = self
            .world
            .insert(
                (berry_bush_sprite,),
                vec![(
                    Position { x: 256.0, y: 256.0 },
                    IsResource {
                        resource_type: ResourceType::Food,
                        resource_stored_amount: 50,
                    },
                )],
            )
            .first()
            .unwrap()
            .clone();

        let villager_1 = self.world.insert(
            (villager_sprite,),
            vec![(
                Position { x: 80.0, y: 64.0 },
                IsResourceGatherer {
                    stroke_gathered_amount: 1,
                    stroke_interval: 5.0,
                    stroke_last_tick: 0,
                    resource_stored: 0,
                    resource_max_stored: 10,
                    resource_type: ResourceType::None,
                },
                GatherTask { target: berry_bush },
                MoveTowardsTargetTask { target: berry_bush },
            )],
        );
    }

    pub fn fixed_update(&mut self) {}

    pub fn update(&mut self) {
        self.update_schedule
            .execute(&mut self.world, &mut self.resources);
    }
}
