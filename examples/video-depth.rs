use bevy::prelude::*;
use bevy_av1::{
    PlaybackMode, VideoPlayer, VideoPlugin, VideoSink, VideoTargetApp, VideoTargetAssets,
};
use displacement::luminance::{
    LuminanceMaterial, LuminancePlugin, LuminanceTextureSource, LuminanceTextureTarget,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, VideoPlugin, LuminancePlugin))
        .init_video_target_asset::<StandardMaterial>()
        .init_video_target_asset::<LuminanceMaterial>()
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_things)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            VideoPlayer::new(
                asset_server.load("textures/tears-of-steel.ivf"),
                PlaybackMode::Loop,
            ),
            Mesh3d(
                meshes.add(
                    Mesh::from(Plane3d::default().mesh())
                        .with_generated_tangents()
                        .unwrap(),
                ),
            ),
            MeshMaterial3d(materials.add(StandardMaterial {
                parallax_depth_scale: 0.07,
                ..default()
            })),
            Rotate,
        ))
        .observe(
            move |trigger: Trigger<OnAdd, VideoSink>,
                  mut commands: Commands,
                  mut sinks: Query<(
                &VideoSink,
                &MeshMaterial3d<StandardMaterial>,
                &mut Transform,
            )>,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  mut video_targets: ResMut<VideoTargetAssets<StandardMaterial>>| {
                let entity = trigger.target();
                if let Ok((sink, mesh_material, mut transform)) = sinks.get_mut(entity)
                    && let Some(material) = materials.get_mut(&mesh_material.0)
                {
                    video_targets.add_target(sink, &mesh_material.0);
                    material.base_color_texture = Some(sink.image().clone());

                    let aspect = sink.width() as f32 / sink.height() as f32;
                    if aspect > 1.0 {
                        transform.scale = Vec3::new(aspect, 1.0, 1.0);
                    } else {
                        transform.scale = Vec3::new(1.0, aspect, 1.0);
                    }

                    commands
                        .spawn(LuminanceTextureSource::new(sink.image().clone()))
                        .observe(
                            move |trigger: Trigger<OnAdd, LuminanceTextureTarget>,
                                  targets: Query<(
                                &LuminanceTextureTarget,
                                &MeshMaterial2d<LuminanceMaterial>,
                            )>,
                                  player: Single<
                                (&VideoSink, &MeshMaterial3d<StandardMaterial>),
                                With<VideoPlayer>,
                            >,
                                  mut materials: ResMut<Assets<StandardMaterial>>,
                                  mut video_targets: ResMut<
                                VideoTargetAssets<LuminanceMaterial>,
                            >| {
                                let entity = trigger.target();
                                if let Ok((target, luminance_mesh_material)) = targets.get(entity)
                                    && let (sink, mesh_material) = *player
                                    && let Some(material) = materials.get_mut(&mesh_material.0)
                                {
                                    video_targets.add_target(sink, &luminance_mesh_material.0);
                                    material.depth_map = Some(target.texture().clone());
                                }
                            },
                        );
                }
            },
        );

    // light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-1.5, 1.5, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[derive(Component)]
struct Rotate;

fn rotate_things(mut q: Query<&mut Transform, With<Rotate>>, time: Res<Time>) {
    for mut t in &mut q {
        t.rotate_y(time.delta_secs());
    }
}
