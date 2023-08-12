use bevy::prelude::*;

use super::TILE_SIZE;

#[derive(Component)]
pub struct TileComponent {
    pub chunk_loc: (i32, i32),
    pub loc: (usize, usize),
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub pixels: [Color; TILE_SIZE * TILE_SIZE],
    pub entity: Option<Entity>,
}

impl Tile {
    pub fn new_fill(color: Color) -> Self {
        Self {
            pixels: [color; TILE_SIZE * TILE_SIZE],
            entity: None,
        }
    }

    pub fn set_pixel(&mut self, pixel: (usize, usize), color: Color) -> anyhow::Result<()> {
        if pixel.0 >= TILE_SIZE || pixel.1 >= TILE_SIZE {
            return Err(anyhow!(
                "pixel should be in range 0, 0 and {0}, {0}",
                TILE_SIZE - 1
            ));
        }

        self.pixels[pixel.0 + pixel.1 * TILE_SIZE] = color;

        Ok(())
    }

    pub fn get_pixel(&mut self, pixel: (usize, usize)) -> anyhow::Result<Color> {
        if pixel.0 >= TILE_SIZE || pixel.1 >= TILE_SIZE {
            return Err(anyhow!(
                "pixel should be in range 0, 0 and {0}, {0}",
                TILE_SIZE - 1
            ));
        }

        Ok(self.pixels[pixel.0 + pixel.1 * TILE_SIZE])
    }
}
