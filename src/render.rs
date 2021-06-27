use crate::physics::Position;
use bevy::prelude::*;

fn position_sprites_system(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation.x = position.0.x;
        transform.translation.y = position.0.y;
    }
}

pub struct SantaRenderPlugin;

impl Plugin for SantaRenderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(position_sprites_system.system().label("position_sprites"));
    }
}
