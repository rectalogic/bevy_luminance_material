use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        mesh::PrimitiveTopology,
        render_resource::{
            AsBindGroup, Extent3d, ShaderRef, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin},
};

const SHADER_ASSET_PATH: &str = "shaders/luminance_material.wgsl";

pub struct LuminancePlugin;

impl Plugin for LuminancePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<LuminanceMaterial>::default())
            .init_resource::<DummyTriangle>()
            .add_systems(Startup, setup)
            .add_systems(PostUpdate, handle_new_sources);
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct LuminanceTextureSource {
    texture: Handle<Image>,
}

impl LuminanceTextureSource {
    pub fn new(texture: Handle<Image>) -> Self {
        Self { texture }
    }
}

#[derive(Component, Default, Debug, Clone)]
#[require(RenderLayers::layer(3))]
pub struct LuminanceTextureTarget {
    texture: Handle<Image>,
}

impl LuminanceTextureTarget {
    pub fn texture(&self) -> &Handle<Image> {
        &self.texture
    }
}

#[derive(Resource, Default)]
struct DummyTriangle(Handle<Mesh>);

fn setup(mut meshes: ResMut<Assets<Mesh>>, mut triangle: ResMut<DummyTriangle>) {
    // Dummy triangle for shader
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_indices(bevy::render::mesh::Indices::U32(vec![0, 1, 2]))
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0.0, 0.0, 0.0]; 3]);
    triangle.0 = meshes.add(mesh);
}

fn handle_new_sources(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    triangle: Res<DummyTriangle>,
    mut materials: ResMut<Assets<LuminanceMaterial>>,
    sources: Query<(Entity, &LuminanceTextureSource), Added<LuminanceTextureSource>>,
) {
    for (entity, source) in &sources {
        let mut image = Image::new_uninit(
            Extent3d {
                //XXX make configurable? or match source texture size once known?
                width: 512,
                height: 512,
                ..default()
            },
            TextureDimension::D2,
            TextureFormat::Bgra8UnormSrgb,
            RenderAssetUsages::default(),
        );
        image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_DST
            | TextureUsages::RENDER_ATTACHMENT;
        let target_image = images.add(image);

        commands.entity(entity).insert((
            Camera2d,
            Camera {
                target: target_image.clone().into(),
                ..default()
            },
            Mesh2d(triangle.0.clone()),
            MeshMaterial2d(materials.add(LuminanceMaterial {
                texture: source.texture.clone(),
            })),
            LuminanceTextureTarget {
                texture: target_image,
            },
        ));
    }
}

#[derive(Asset, TypePath, AsBindGroup, Default, Debug, Clone)]
pub struct LuminanceMaterial {
    #[texture(1)]
    #[sampler(2)]
    texture: Handle<Image>,
}

impl Material2d for LuminanceMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
