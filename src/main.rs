use bevy::{
    color::palettes::basic::RED, pbr::MaterialExtension, prelude::*, render::render_resource::*,
};

/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "shaders/vertexture.wgsl";

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,))
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
    let texture = asset_server.load("textures/bevy_logo.png");
    let scale = Vec3::new(4.0, 1.0, 1.0); // based on image aspect ratio

    commands.spawn((
        Mesh3d(
            meshes.add(
                Mesh::from(Plane3d::default().mesh())
                    .with_generated_tangents()
                    .unwrap(),
            ),
        ),
        Transform::from_scale(scale),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: RED.into(),
            base_color_texture: Some(texture.clone()),
            depth_map: Some(texture),
            parallax_depth_scale: 0.09,
            ..default()
        })),
    ));

    // light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        Rotate,
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[derive(Component)]
struct Rotate;

fn rotate_things(mut q: Query<&mut Transform, With<Rotate>>, time: Res<Time>) {
    for mut t in &mut q {
        t.rotate_y(time.delta_secs());
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Default, Clone)]
struct VertextureExtension {
    #[uniform(100)]
    _dummy: f32,
}

impl MaterialExtension for VertextureExtension {
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
