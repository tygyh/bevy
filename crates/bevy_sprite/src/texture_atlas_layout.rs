use bevy_asset::Handle;
use bevy_math::{Rect, Vec2};
use bevy_reflect::{FromReflect, Reflect, TypeUuid};
use bevy_render::texture::Image;
use bevy_utils::HashMap;

/// Maps a layout for a texture. Used with the [`TextureAtlas`] component it allows to
/// either draw a specific area of the target texture, or to animate a sprite sheet.
///
/// Optionaly it can store a mapping from sub texture handles to the related area index (see
/// [`TextureAtlasBuilder`]).
///
/// [Example usage animating sprite.](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_sheet.rs)
/// [Example usage loading sprite sheet.](https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs)
///
/// [`TextureAtlas`]: crate::TextureAtlas
/// [`TextureAtlasBuilder`]: crate::TextureAtlasBuilder
#[derive(Reflect, FromReflect, Debug, Clone, TypeUuid)]
#[uuid = "7233c597-ccfa-411f-bd59-9af349432ada"]
#[reflect(Debug)]
pub struct TextureAtlasLayout {
    // TODO: add support to Uniforms derive to write dimensions and sprites to the same buffer
    pub size: Vec2,
    /// The specific areas of the atlas where each texture can be found
    pub textures: Vec<Rect>,
    /// Texture handle to area index mapping. Set by [`TextureAtlasBuilder`].
    ///
    /// [`TextureAtlasBuilder`]: crate::TextureAtlasBuilder
    pub texture_handles: Option<HashMap<Handle<Image>, usize>>,
}

impl TextureAtlasLayout {
    /// Create a new empty layout with custom `dimensions`
    pub fn new_empty(dimensions: Vec2) -> Self {
        Self {
            size: dimensions,
            texture_handles: None,
            textures: Vec::new(),
        }
    }

    /// Generate a [`TextureAtlasLayout`] as a grid where each
    /// `tile_size` by `tile_size` grid-cell is one of the *section* in the
    /// atlas. Grid cells are separated by some `padding`, and the grid starts
    /// at `offset` pixels from the top left corner. Resulting layout is
    /// indexed left to right, top to bottom.
    ///
    /// # Arguments
    ///
    /// * `tile_size` - Each layout grid cell size
    /// * `columns` - Grid column count
    /// * `rows` - Grid row count
    /// * `padding` - Optional padding between cells
    /// * `offset` - Optional global grid offset
    pub fn from_grid(
        tile_size: Vec2,
        columns: usize,
        rows: usize,
        padding: Option<Vec2>,
        offset: Option<Vec2>,
    ) -> Self {
        let padding = padding.unwrap_or_default();
        let offset = offset.unwrap_or_default();
        let mut sprites = Vec::new();
        let mut current_padding = Vec2::ZERO;

        for y in 0..rows {
            if y > 0 {
                current_padding.y = padding.y;
            }
            for x in 0..columns {
                if x > 0 {
                    current_padding.x = padding.x;
                }

                let cell = Vec2::new(x as f32, y as f32);

                let rect_min = (tile_size + current_padding) * cell + offset;

                sprites.push(Rect {
                    min: rect_min,
                    max: rect_min + tile_size,
                });
            }
        }

        let grid_size = Vec2::new(columns as f32, rows as f32);

        Self {
            size: ((tile_size + current_padding) * grid_size) - current_padding,
            textures: sprites,
            texture_handles: None,
        }
    }

    /// Add a *section* to the list in the layout and returns its index
    /// which can be used with [`TextureAtlas`]
    ///
    /// # Arguments
    ///
    /// * `rect` - The section of the texture to be added
    pub fn add_texture(&mut self, rect: Rect) -> usize {
        self.textures.push(rect);
        self.textures.len() - 1
    }

    /// How many textures are in the `TextureAtlas`
    pub fn len(&self) -> usize {
        self.textures.len()
    }

    pub fn is_empty(&self) -> bool {
        self.textures.is_empty()
    }

    /// Retrieves the texture *section* index of the given `texture` handle.
    ///
    /// This requires the layout to have been built using a [`TextureAtlasBuilder`]
    ///
    /// [`TextureAtlasBuilder`]: crate::TextureAtlasBuilder
    pub fn get_texture_index(&self, texture: &Handle<Image>) -> Option<usize> {
        self.texture_handles
            .as_ref()
            .and_then(|texture_handles| texture_handles.get(texture).cloned())
    }
}
