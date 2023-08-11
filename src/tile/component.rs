use super::TILE_SIZE;
use bevy::{prelude::*, render::render_resource::Extent3d};

#[derive(Component, Clone, Debug)]
pub struct Tile {
    image: Image,
    handle: Handle<Image>,
}

impl Tile {
    pub fn new_solid(color: Color, image_server: &mut ResMut<Assets<Image>>) -> Self {
        let image = Image::new_fill(
            Extent3d {
                width: TILE_SIZE as u32,
                height: TILE_SIZE as u32,
                ..Default::default()
            },
            bevy::render::render_resource::TextureDimension::D2,
            &color.as_rgba_u8(),
            bevy::render::render_resource::TextureFormat::Rgba8Unorm,
        );

        Self {
            image: image.clone(),
            handle: image_server.add(image),
        }
    }

    /// Sets the pixel at the given location, returning an error if it was unable to set
    pub fn set_pixel(&mut self, pixel: (usize, usize), color: Color) -> anyhow::Result<()> {
        if pixel.0 > TILE_SIZE || pixel.1 > TILE_SIZE {
            return Err(anyhow!(
                "Expected pixel between 0, 0 and {TILE_SIZE}, {TILE_SIZE}, but got pixel {}, {}",
                pixel.0,
                pixel.1
            ));
        }

        let rgba_u8_color = color.as_rgba_u8();
        let pixel_index = (pixel.0 + pixel.1 * TILE_SIZE) * 4;

        self.image.data[pixel_index] = rgba_u8_color[0];
        self.image.data[pixel_index + 1] = rgba_u8_color[1];
        self.image.data[pixel_index + 2] = rgba_u8_color[2];
        self.image.data[pixel_index + 3] = rgba_u8_color[3];

        Ok(())
    }

    /// Updates the image handle to be the updated version of the tile.
    pub fn update_image(&mut self, image_server: &mut ResMut<Assets<Image>>) {
        self.handle = image_server.set(self.handle.clone_weak(), self.image.clone());
    }

    pub fn get_image_handle(&self) -> Handle<Image> {
        self.handle.clone_weak()
    }
}
