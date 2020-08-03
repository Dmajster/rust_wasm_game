extern crate console_error_panic_hook;
use std::panic;

mod components;
use components::*;

mod systems;
use systems::*;

use components::{
    IsRenderable, IsResourceDropOff, IsResourceGatherer, IsResourceSource, MoveToEntityTask,
    ResourceGatherTask,
};
use legion::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::{console, HtmlCanvasElement};

#[wasm_bindgen]
pub struct Game {
    world: World,
    resources: Resources,
    update_schedule: Schedule,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Renderer {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct Clock {
    pub last_frame_instant: f64,
    pub time_delta: f32,
}

#[wasm_bindgen]
impl Clock {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            last_frame_instant: 0.0,
            time_delta: 0.0,
        }
    }
}

pub fn print(text: String) {
    console::log_1(&text.into());
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        panic::set_hook(Box::new(console_error_panic_hook::hook));

        let schedule = Schedule::builder()
            .add_system(resource_gather_task_system())
            .add_system(resource_find_drop_off_task_system())
            .add_system(resource_drop_off_task_system())
            .add_system(move_to_entity_task_system())
            .add_system(move_to_position_task_system())
            .add_thread_local(render_system())
            .build();

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let mut resources = Resources::default();
        resources.insert(Renderer { canvas, context });
        resources.insert(Clock {
            last_frame_instant: 0.0,
            time_delta: 0.001,
        });

        Self {
            world: World::new(),
            resources: resources,
            update_schedule: schedule,
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
            dimensions: (8, 16),
        };

        let town_center = self.world.push((
            town_center_sprite,
            Position::new(64.0, 64.0),
            IsResourceDropOff {},
        ));

        let berry_bush = self.world.push((
            berry_bush_sprite,
            Position::new(256.0, 256.0),
            IsResourceSource {
                resource_type: ResourceType::Food,
                resource_left: 100,
            },
        ));

        let villager_1 = self.world.push((
            villager_sprite,
            Position::new(384.0, 384.0),
            IsResourceGatherer {
                stroke_gather_amount: 10,
                stroke_interval: 0.5,
                stroke_last_tick: 0,
                resource_type: ResourceType::None,
                resource_stored: 0,
                resource_max: 30,
            },
            MoveToEntityTask { entity: berry_bush },
            ResourceGatherTask { target: berry_bush },
        ));
    }

    pub fn set_clock(&mut self, clock: &Clock) {
        let mut resource_clock = self.resources.get_mut::<Clock>().unwrap();
        resource_clock.time_delta = clock.time_delta;
        resource_clock.last_frame_instant = clock.last_frame_instant;
    }

    pub fn fixed_update(&mut self) {}

    pub fn update(&mut self) {
        self.update_schedule
            .execute(&mut self.world, &mut self.resources);
    }
}
