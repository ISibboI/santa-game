use crate::levels::LevelCameraBoundary;
use crate::physics::Position;
use crate::player::Santa;
use bevy::prelude::*;
use bevy::render::camera::{
    camera_system, Camera, CameraProjection, DepthCalculation, VisibleEntities,
};

pub struct SantaOrthoProjection {
    pub projection_matrix: Mat4,
    pub viewport_dimensions: Rect<f32>,
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

        self.viewport_dimensions = Rect {
            left: -width / 2.0,
            right: width / 2.0,
            bottom: -height / 2.0,
            top: height / 2.0,
        };
        self.projection_matrix = Mat4::orthographic_rh(
            self.viewport_dimensions.left,
            self.viewport_dimensions.right,
            self.viewport_dimensions.bottom,
            self.viewport_dimensions.top,
            0.0,
            1000.0,
        );
    }

    fn depth_calculation(&self) -> DepthCalculation {
        DepthCalculation::ZDifference
    }
}

impl Default for SantaOrthoProjection {
    fn default() -> Self {
        let mut result = Self {
            projection_matrix: Default::default(),
            viewport_dimensions: Default::default(),
        };
        result.update(800.0, 600.0);
        result
    }
}

fn init_camera_system(mut commands: Commands) {
    let projection = SantaOrthoProjection::default();
    let cam_name = bevy::render::render_graph::base::camera::CAMERA_2D;
    let mut camera = Camera::default();
    camera.name = Some(cam_name.to_owned());

    commands.spawn_bundle((
        Transform::from_translation(Vec3::new(0.0, 0.0, 999.0)),
        GlobalTransform::default(),
        VisibleEntities::default(),
        camera,
        projection,
    ));
}

fn follow_player_camera_system(
    mut camera_query: Query<(&mut Transform, &SantaOrthoProjection), With<Camera>>,
    player_query: Query<&Position, With<Santa>>,
    camera_boundary: Res<LevelCameraBoundary>,
) {
    for (mut camera_transform, santa_ortho_projection) in camera_query.iter_mut() {
        for player_position in player_query.iter() {
            camera_transform.translation.x = player_position
                .0
                .x
                .max(camera_boundary.0.left - santa_ortho_projection.viewport_dimensions.left)
                .min(camera_boundary.0.right - santa_ortho_projection.viewport_dimensions.right);
            camera_transform.translation.y = player_position
                .0
                .y
                .max(camera_boundary.0.bottom - santa_ortho_projection.viewport_dimensions.bottom)
                .min(camera_boundary.0.top - santa_ortho_projection.viewport_dimensions.top);
        }
    }
}

pub struct SantaCameraPlugin;

impl Plugin for SantaCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_camera_system.system())
            .add_system(
                camera_system::<SantaOrthoProjection>
                    .system()
                    .label("camera_system")
                    .after("position_sprites"),
            )
            .add_system(
                follow_player_camera_system
                    .system()
                    .label("follow_player_camera")
                    .after("camera_system"),
            );
    }
}
