use bevy::{
    asset::RenderAssetUsages,
    math::FloatOrd,
    prelude::*,
    render::{
        camera::{ImageRenderTarget, RenderTarget},
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
            .add_systems(PostUpdate, handle_new_sources);
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct LuminanceTextureSource {
    pub texture: Handle<Image>,
    pub render_layer: usize,
}

impl LuminanceTextureSource {
    pub fn new(texture: Handle<Image>, render_layer: usize) -> Self {
        Self {
            texture,
            render_layer,
        }
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct LuminanceTextureTarget {
    texture: Handle<Image>,
}

impl LuminanceTextureTarget {
    pub fn texture(&self) -> &Handle<Image> {
        &self.texture
    }
}

fn handle_new_sources(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<LuminanceMaterial>>,
    sources: Query<(Entity, &LuminanceTextureSource), Added<LuminanceTextureSource>>,
) {
    for (entity, source) in &sources {
        // XXX update to Image::new_target_texture
        let mut image = Image::new_uninit(
            Extent3d {
                //XXX make configurable? or match source texture size once known?
                width: 512,
                height: 512,
                ..default()
            },
            TextureDimension::D2,
            TextureFormat::Bgra8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD,
        );
        image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_DST
            | TextureUsages::RENDER_ATTACHMENT;
        let target_image = images.add(image);

        commands.entity(entity).insert((
            Camera {
                target: RenderTarget::Image(ImageRenderTarget {
                    handle: target_image.clone(),
                    scale_factor: FloatOrd(1.0),
                }),
                ..default()
            },
            RenderLayers::layer(source.render_layer),
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
struct LuminanceMaterial {
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
