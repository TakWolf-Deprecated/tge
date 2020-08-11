use super::{opengl, Filter, Texture};
use crate::error::{GameError, GameResult};
use crate::math::{Position, Size, Region};
use crate::engine::Engine;
use ab_glyph::{Font as AbGlyphFont, FontVec, Rect, PxScaleFactor, OutlinedGlyph};
use std::path::Path;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct GlyphId(ab_glyph::GlyphId);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GlyphDrawInfo {
    pub bounds: Region,
    pub uv: Region,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LineMetrics {
    pub ascent: f32,
    pub descent: f32,
    pub height: f32,
    pub line_gap: f32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GlyphMetrics {
    pub advance_width: f32,
}

pub enum CachedBy {
    Added(Option<GlyphDrawInfo>),
    Existed(Option<GlyphDrawInfo>),
}

pub enum CacheError {
    TooLarge,
    NoRoom,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct CacheKey {
    character: char,
    px: u32,
}

struct Cache {
    texture: Texture,
    texture_size: u32,
    draw_infos: HashMap<CacheKey, Option<(Rect, Region)>>,
    rows: Vec<Size<u32>>,
}

pub struct Font {
    font: FontVec,
    glyph_ids: RefCell<HashMap<char, GlyphId>>,
    cache: RefCell<Cache>,
    fit_hidpi: bool,
}

impl Font {
    pub(crate) fn new(engine: &mut Engine, bytes: Vec<u8>, cache_texture_size: u32) -> GameResult<Self> {
        let font = FontVec::try_from_vec(bytes)
            .map_err(|error| GameError::InitError(error.into()))?;
        let cache_texture = Texture::for_font_cache(engine, cache_texture_size)?;
        let cache = Cache {
            texture: cache_texture,
            texture_size: cache_texture_size,
            draw_infos: HashMap::new(),
            rows: Vec::new(),
        };
        Ok(Self {
            font,
            glyph_ids: RefCell::new(HashMap::new()),
            cache: RefCell::new(cache),
            fit_hidpi: true,
        })
    }

    pub fn from_bytes(engine: &mut Engine, bytes: Vec<u8>) -> GameResult<Self> {
        Self::new(engine, bytes, 1024)
    }

    pub fn load(engine: &mut Engine, path: impl AsRef<Path>) -> GameResult<Self> {
        let bytes = engine.filesystem().read(path)?;
        Self::from_bytes(engine, bytes)
    }

    pub(crate) fn glyph_id(&self, character: char) -> GlyphId {
        let mut glyph_ids = self.glyph_ids.borrow_mut();
        let glyph_id = glyph_ids.entry(character).or_insert_with(|| GlyphId(self.font.glyph_id(character)));
        *glyph_id
    }

    fn scale_factor(&self, px: f32) -> f32 {
        px / self.font.units_per_em().expect("no units per em")
    }

    pub(crate) fn line_metrics(&self, px: f32) -> LineMetrics {
        let scale_factor = self.scale_factor(px);
        LineMetrics {
            ascent: self.font.ascent_unscaled() * scale_factor,
            descent: self.font.descent_unscaled() * scale_factor,
            height: self.font.height_unscaled() * scale_factor,
            line_gap: self.font.line_gap_unscaled() * scale_factor,
        }
    }

    pub(crate) fn glyph_metrics(&self, glyph_id: GlyphId, px: f32) -> GlyphMetrics {
        let scale_factor = self.scale_factor(px);
        GlyphMetrics {
            advance_width: self.font.h_advance_unscaled(glyph_id.0) * scale_factor,
        }
    }

    pub(crate) fn clone_cache_texture(&self) -> Rc<opengl::Texture> {
        self.cache.borrow().texture.texture().clone()
    }

    pub(crate) fn cache_texture_size(&self) -> u32 {
        self.cache.borrow().texture_size
    }

    pub(crate) fn cache_glyph(&self, character: char, px: f32, graphics_scale_factor: f32) -> Result<CachedBy, CacheError> {
        let px = (px * graphics_scale_factor).round();
        let cache_key = CacheKey { character, px: px as u32 };
        let mut cache = self.cache.borrow_mut();
        if let Some(draw_info) = cache.draw_infos.get(&cache_key) {
            let draw_info = draw_info.map(|(px_bounds, uv)| {
                let bounds = Region::min_max(
                    Position::new(px_bounds.min.x / graphics_scale_factor, px_bounds.min.y / graphics_scale_factor),
                    Position::new(px_bounds.max.x / graphics_scale_factor, px_bounds.max.y / graphics_scale_factor),
                );
                GlyphDrawInfo { bounds, uv }
            });
            return Ok(CachedBy::Existed(draw_info));
        }
        let outline_glyph = {
            let glyph = self.glyph_id(character);
            self.font.outline(glyph.0).map(|outline| {
                let scale_factor = self.scale_factor(px);
                OutlinedGlyph::new(glyph.0.with_scale(0.0), outline, PxScaleFactor {
                    horizontal: scale_factor,
                    vertical: scale_factor,
                })
            })
        };
        if let Some(outlined_glyph) = outline_glyph {
            let px_bounds = outlined_glyph.px_bounds();
            let glyph_size = Size::new(px_bounds.width().ceil() as u32, px_bounds.height().ceil() as u32);
            let glyph_cache_size = Size::new(glyph_size.width + 1, glyph_size.height + 1);
            let cache_texture_size = cache.texture_size;
            if glyph_cache_size.width > cache_texture_size || glyph_cache_size.height > cache_texture_size {
                return Err(CacheError::TooLarge);
            }
            let mut region = None;
            let mut row_bottom = 0;
            for row in cache.rows.iter_mut() {
                if glyph_cache_size.height <= row.height && glyph_cache_size.width <= cache_texture_size - row.width {
                    region = Some(Region::new(row.width, row_bottom, glyph_size.width, glyph_size.height));
                    row.width += glyph_cache_size.width;
                    break;
                }
                row_bottom += row.height;
            }
            if region.is_none() {
                if glyph_cache_size.height <= cache_texture_size - row_bottom {
                    region = Some(Region::new(0, row_bottom, glyph_size.width, glyph_size.height));
                    cache.rows.push(glyph_cache_size);
                }
            }
            if let Some(region) = region {
                let mut pixels = vec![255; (glyph_size.width * glyph_size.height * 4) as usize];
                outlined_glyph.draw(|x, y, c| {
                    let color = (c * 255.0) as u8;
                    let index = ((x + y * glyph_size.width) * 4) as usize;
                    pixels[index + 3] = color;
                });
                cache.texture.update_pixels(region, Some(&pixels))
                    .expect("update font cache texture error");
                let uv = {
                    let texture_size = {
                        let texture_size = cache.texture.size();
                        Size::new(texture_size.width as f32, texture_size.height as f32)
                    };
                    Region::new(
                        region.x as f32 / texture_size.width,
                        region.y as f32 / texture_size.height,
                        region.width as f32 / texture_size.width,
                        region.height as f32 / texture_size.height,
                    )
                };
                cache.draw_infos.insert(cache_key, Some((px_bounds, uv)));
                let draw_info = {
                    let bounds = Region::min_max(
                        Position::new(px_bounds.min.x / graphics_scale_factor, px_bounds.min.y / graphics_scale_factor),
                        Position::new(px_bounds.max.x / graphics_scale_factor, px_bounds.max.y / graphics_scale_factor),
                    );
                    GlyphDrawInfo { bounds, uv }
                };
                Ok(CachedBy::Added(Some(draw_info)))
            } else {
                Err(CacheError::NoRoom)
            }
        } else {
            cache.draw_infos.insert(cache_key, None);
            Ok(CachedBy::Added(None))
        }
    }

    pub(crate) fn clear_cache(&self) {
        let mut cache = self.cache.borrow_mut();
        cache.draw_infos.clear();
        cache.rows.clear();
    }

    pub(crate) fn resize_cache(&self, cache_texture_size: u32) {
        let mut cache = self.cache.borrow_mut();
        cache.draw_infos.clear();
        cache.rows.clear();
        cache.texture.init_pixels((cache_texture_size, cache_texture_size), None)
            .expect("resize font cache texture error");
        cache.texture_size = cache_texture_size;
    }

    pub fn filter(&self) -> Filter {
        self.cache.borrow().texture.filter()
    }

    pub fn set_filter(&mut self, filter: Filter) {
        self.cache.borrow_mut().texture.set_filter(filter)
    }

    pub fn is_fit_hidpi(&self) -> bool {
        self.fit_hidpi
    }

    pub fn set_fit_hidpi(&mut self, fit_hidpi: bool) {
        self.fit_hidpi = fit_hidpi;
    }
}
