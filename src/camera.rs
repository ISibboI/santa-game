use bevy::prelude::*;
use bevy::render::camera::{CameraProjection, DepthCalculation, Camera, camera_system, VisibleEntities};

pub struct SantaOrthoProjection {
    projection_matrix: Mat4,
}

impl CameraProjection for SantaOrthoProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        self.projection_matrix.clone()
    }

    fn update(&mut self, mut width: f32, mut height: f32) {
        const MIN_SCALE: f32 = 4.0;
        const BACKGROUND_HEIGHT: f32 = 200.0;

        width /= MIN_SCALE;
        height /= MIN_SCALE;

        if height > BACKGROUND_HEIGHT {
            let scale_factor = BACKGROUND_HEIGHT / height;
            width *= scale_factor;
            height *= scale_factor;
        }

        self.projection_matrix = Mat4::orthographic_rh(-width / 2.0, width / 2.0, -height / 2.0, height / 2.0, 0.0, 1000.0);
    }

    fn depth_calculation(&self) -> DepthCalculation {
        DepthCalculation::ZDifference
    }
}

impl Default for SantaOrthoProjection {
    fn default() -> Self {
        let mut result = Self {projection_matrix: Default::default()};
        result.update(800.0, 600.0);
        result
    }
}

fn init_camera_system(mut commands: Commands) {
    let projection = SantaOrthoProjection::default();
    let cam_name = bevy::render::render_graph::base::camera::CAMERA_2D;
    let mut camera = Camera::default();
    camera.name = Some(cam_name.to_owned());

    commands.spawn_bundle((Transform::from_translation(Vec3::new(0.0, 0.0, 999.0)),
    GlobalTransform::default(),
    VisibleEntities::default(),
    camera,
    projection));
}

pub struct SantaCameraPlugin;

impl Plugin for SantaCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_camera_system.system())
            .add_system_to_stage(CoreStage::PostUpdate, camera_system::<SantaOrthoProjection>.system());
    }
}