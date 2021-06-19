use crate::physics::{Position, Speed};
use bevy::prelude::*;

fn position_sprite_system(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation.x = position.0.x;
        transform.translation.y = position.0.y;
    }
}

pub struct SantaRenderPlugin;

impl Plugin for SantaRenderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(CoreStage::PostUpdate, position_sprite_system.system());
    }
}
