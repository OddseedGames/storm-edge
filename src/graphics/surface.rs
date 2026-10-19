use bevy::{
    image::{
        ImageAddressMode, ImageFilterMode, ImageLoaderSettings, ImageSampler,
        ImageSamplerDescriptor,
    },
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::{AsBindGroup, TextureFormat},
    shader::ShaderRef,
};

pub type IslandMaterial = ExtendedMaterial<StandardMaterial, RockSurface>;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct RockSurface {
    #[texture(100)]
    #[sampler(101)]
    pub albedo: Handle<Image>,
    #[uniform(102)]
    pub parameters: Vec4,
}

impl MaterialExtension for RockSurface {
    fn fragment_shader() -> ShaderRef {
        "shaders/rock.wgsl".into()
    }
}

#[derive(Resource)]
pub struct SurfaceTexture {
    handle: Handle<Image>,
    mipped: bool,
}

pub fn load(assets: &AssetServer, commands: &mut Commands) -> Handle<Image> {
    let handle = assets
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::MirrorRepeat,
                address_mode_v: ImageAddressMode::MirrorRepeat,
                mag_filter: ImageFilterMode::Linear,
                min_filter: ImageFilterMode::Linear,
                mipmap_filter: ImageFilterMode::Linear,
                anisotropy_clamp: 8,
                ..default()
            });
        })
        .load("textures/weathered-rock.png");
    commands.insert_resource(SurfaceTexture {
        handle: handle.clone(),
        mipped: false,
    });
    handle
}

pub fn prepare_mips(texture: Option<ResMut<SurfaceTexture>>, mut images: ResMut<Assets<Image>>) {
    let Some(mut texture) = texture else {
        return;
    };
    if texture.mipped {
        return;
    }
    let Some(mut image) = images.get_mut(&texture.handle) else {
        return;
    };
    if image.texture_descriptor.format != TextureFormat::Rgba8UnormSrgb {
        return;
    }
    let Some(mut previous) = image.data.clone() else {
        return;
    };
    let mut width = image.width();
    let mut height = image.height();
    let mut data = previous.clone();
    let mut levels = 1;
    while width > 1 || height > 1 {
        let next_width = (width / 2).max(1);
        let next_height = (height / 2).max(1);
        let mut next = Vec::with_capacity((next_width * next_height * 4) as usize);
        for y in 0..next_height {
            for x in 0..next_width {
                for channel in 0..4 {
                    let mut sum = 0u32;
                    for dy in 0..2 {
                        for dx in 0..2 {
                            let index = (((y * 2 + dy).min(height - 1) * width
                                + (x * 2 + dx).min(width - 1))
                                * 4
                                + channel) as usize;
                            sum += previous[index] as u32;
                        }
                    }
                    next.push((sum / 4) as u8);
                }
            }
        }
        data.extend_from_slice(&next);
        previous = next;
        width = next_width;
        height = next_height;
        levels += 1;
    }
    image.data = Some(data);
    image.texture_descriptor.mip_level_count = levels;
    texture.mipped = true;
}
