use crate::{components::traits::IsRenderable, Position, Renderer};
use legion::{system, world::SubWorld, IntoQuery, Read};

#[system]
#[read_component(Position)]
#[read_component(IsRenderable)]
pub fn render(#[resource] renderer: &Renderer, world: &SubWorld) {
    let mut query = <(Read<Position>, Read<IsRenderable>)>::query();

    renderer.context.clear_rect(
        0.0,
        0.0,
        renderer.canvas.width() as f64,
        renderer.canvas.height() as f64,
    );
    for (position, renderable) in query.iter(world) {
        renderer.context.begin_path();

        renderer.context.set_fill_style(
            &format!(
                "rgb({},{},{})",
                renderable.color.0, renderable.color.1, renderable.color.2
            )
            .into(),
        );

        renderer.context.rect(
            position.x as f64,
            position.y as f64,
            renderable.dimensions.0 as f64,
            renderable.dimensions.1 as f64,
        );

        renderer.context.fill();
    }
}
