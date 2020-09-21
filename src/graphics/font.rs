use super::{Graphics, opengl, Filter, Texture};
use crate::error::{GameError, GameResult};
use crate::math::{Size, Region};
use crate::engine::Engine;
use fontdue::{FontSettings, Metrics};
use std::path::Path;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GlyphDrawInfo {
    pub uv_bounds: Region,
    pub uv: Region,
}

impl GlyphDrawInfo {
    fn new(metrics: Metrics, hidpi_scale_factor: f32, uv: Region) -> Self {
        let uv_bounds = Region::new(
            metrics.xmin as f32 / hidpi_scale_factor,
            -(metrics.height as f32 + metrics.ymin as f32) / hidpi_scale_factor,
            metrics.width as f32 / hidpi_scale_factor,
            metrics.height as f32 / hidpi_scale_factor,
        );
        Self {
            uv_bounds,
            uv,
        }
    }
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
    Added(GlyphDrawInfo),
    Existed(GlyphDrawInfo),
}

pub enum CacheError {
    TooLarge,
    NoRoom,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct CacheKey {
    c: char,
    px: u32,
}

struct Cache {
    texture: Texture,
    texture_size: u32,
    draw_infos: HashMap<CacheKey, (Metrics, Region)>,
    rows: Vec<Size<u32>>,
}

pub struct Font {
    font: fontdue::Font,
    cache: RefCell<Cache>,
    hidpi_scale_factor: Option<f32>,
}

impl Font {
    pub(crate) fn new(graphics: &mut Graphics, bytes: &[u8], cache_texture_size: u32) -> GameResult<Self> {
        let font = fontdue::Font::from_bytes(bytes, FontSettings::default())
            .map_err(|error| GameError::InitError(error.into()))?;
        let cache_texture = Texture::for_font_cache(graphics, cache_texture_size)?;
        let cache = Cache {
            texture: cache_texture,
            texture_size: cache_texture_size,
            draw_infos: HashMap::new(),
            rows: Vec::new(),
        };
        Ok(Self {
            font,
            cache: RefCell::new(cache),
            hidpi_scale_factor: None,
        })
    }

    pub fn from_bytes(graphics: &mut Graphics, bytes: &[u8]) -> GameResult<Self> {
        Self::new(graphics, bytes, 1024)
    }

    pub fn load(engine: &mut Engine, path: impl AsRef<Path>) -> GameResult<Self> {
        let bytes = engine.filesystem().read(path)?;
        Self::from_bytes(engine.graphics(), &bytes)
    }

    pub(crate) fn line_metrics(&self, px: f32) -> LineMetrics {
        let horizontal_line_metrics = self.font.horizontal_line_metrics(px)
            .expect("no horizontal line metrics");
        LineMetrics {
            ascent: horizontal_line_metrics.ascent,
            descent: horizontal_line_metrics.descent,
            height: horizontal_line_metrics.ascent - horizontal_line_metrics.descent,
            line_gap: horizontal_line_metrics.line_gap,
        }
    }

    pub(crate) fn glyph_metrics(&self, c: char, px: f32) -> GlyphMetrics {
        let metrics = self.font.metrics(c, px);
        GlyphMetrics {
            advance_width: metrics.advance_width,
        }
    }

    pub(crate) fn cache_texture(&self) -> Rc<opengl::Texture> {
        self.cache.borrow().texture.texture().clone()
    }

    pub(crate) fn cache_texture_size(&self) -> u32 {
        self.cache.borrow().texture_size
    }

    pub(crate) fn cache_glyph(&self, c: char, px: f32, hidpi_scale_factor: f32) -> Result<CachedBy, CacheError> {
        let px = (px * hidpi_scale_factor).round();
        let cache_key = CacheKey { c, px: px as u32 };
        let mut cache = self.cache.borrow_mut();
        if let Some((metrics, uv)) = cache.draw_infos.get(&cache_key) {
            let draw_info = GlyphDrawInfo::new(*metrics, hidpi_scale_factor, *uv);
            return Ok(CachedBy::Existed(draw_info));
        }
        let metrics = self.font.metrics(c, px);
        let glyph_size = Size::new(metrics.width as u32, metrics.height as u32);
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
            let (_, bitmap) = self.font.rasterize(c, px);
            let mut pixels = Vec::with_capacity(bitmap.len() * 4);
            for alpha in bitmap {
                pixels.push(255);
                pixels.push(255);
                pixels.push(255);
                pixels.push(alpha);
            }
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
            cache.draw_infos.insert(cache_key, (metrics, uv));
            let draw_info = GlyphDrawInfo::new(metrics, hidpi_scale_factor, uv);
            Ok(CachedBy::Added(draw_info))
        } else {
            Err(CacheError::NoRoom)
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
        cache.texture.resize((cache_texture_size, cache_texture_size));
        cache.texture_size = cache_texture_size;
    }

    pub fn filter(&self) -> Filter {
        self.cache.borrow().texture.filter()
    }

    pub fn set_filter(&mut self, filter: Filter) {
        self.cache.borrow_mut().texture.set_filter(filter)
    }

    pub fn hidpi_scale_factor(&self) -> Option<f32> {
        self.hidpi_scale_factor
    }

    pub fn set_hidpi_scale_factor(&mut self, hidpi_scale_factor: Option<f32>) {
        self.hidpi_scale_factor = hidpi_scale_factor;
    }
}
