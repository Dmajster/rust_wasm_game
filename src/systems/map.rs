use crate::{
    components::{IsRenderable, Position},
    map::Map,
};
use legion::{system, systems::CommandBuffer, world::SubWorld};

#[system]
pub fn map_create(
    #[resource] map_data: &Map,
    world: &SubWorld,
    command_buffer: &mut CommandBuffer,
) {
    let grass_tile = IsRenderable {
        color: (0, 128, 0),
        dimensions: (32, 32),
    };

    let map_width = map_data.dimensions.0;

    for (index, tile_type) in map_data.tiles.iter().enumerate() {
        let position = Position::new(
            (index as usize % map_width as usize) as f32 * 32.0,
            (index as usize / map_width as usize) as f32 * 32.0,
        );

        if *tile_type == 0 {
            command_buffer.push((position, grass_tile));
        }
    }
}
