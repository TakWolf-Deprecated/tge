use super::{Filter, Wrap};
use glow::{Context, HasContext, PixelUnpackData};
use std::rc::Rc;

pub type TextureId = <Context as HasContext>::Texture;

pub struct Texture {
    gl: Rc<Context>,
    id: TextureId,
}

impl Texture {
    pub fn new(gl: Rc<Context>) -> Result<Self, String> {
        let id = unsafe {
            gl.create_texture()?
        };
        Ok(Self { gl, id })
    }

    pub fn id(&self) -> TextureId {
        self.id
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.bind_texture(glow::TEXTURE_2D, Some(self.id));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }

    pub fn init_image(&self, width: u32, height: u32, pixels: Option<&[u8]>) {
        unsafe {
            self.gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width as i32,
                height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                pixels,
            );
        }
    }

    pub fn sub_image(&self, offset_x: u32, offset_y: u32, width: u32, height: u32, pixels: Option<&[u8]>) {
        unsafe {
            self.gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                offset_x as i32,
                offset_y as i32,
                width as i32,
                height as i32,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                match pixels {
                    Some(pixels) => PixelUnpackData::Slice(pixels),
                    None => PixelUnpackData::BufferOffset(0),
                },
            );
        }
    }

    pub fn set_filter(&self, filter: Filter) {
        unsafe {
            self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, filter.to_min_flag() as i32);
            self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, filter.to_mag_flag() as i32);
        }
    }

    pub fn generate_mipmap(&self) {
        unsafe {
            self.gl.generate_mipmap(glow::TEXTURE_2D);
        }
    }

    pub fn set_wrap(&self, wrap: Wrap) {
        unsafe {
            self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, wrap.horizontal.to_flag() as i32);
            self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, wrap.vertical.to_flag() as i32);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_texture(self.id);
        }
    }
}

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
