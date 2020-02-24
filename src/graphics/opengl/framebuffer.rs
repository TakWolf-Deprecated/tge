use super::{TextureId, Attachment};
use glow::{Context, HasContext};
use std::rc::Rc;

pub type FramebufferId = <Context as HasContext>::Framebuffer;

pub struct Framebuffer {
    gl: Rc<Context>,
    id: FramebufferId,
}

impl Framebuffer {

    pub fn new(gl: Rc<Context>) -> Result<Self, String> {
        let id = unsafe {
            gl.create_framebuffer()?
        };
        Ok(Self { gl, id })
    }

    pub fn id(&self) -> FramebufferId {
        self.id
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.id));
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }
    }

    pub fn attach_texture(&self, attachment: Attachment, texture_id: Option<TextureId>) {
        unsafe {
            self.gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                attachment.to_flag(),
                glow::TEXTURE_2D,
                texture_id,
                0,
            );
        }
    }

    pub fn check_status(&self) -> Result<(), String> {
        let status = unsafe {
            self.gl.check_framebuffer_status(glow::FRAMEBUFFER)
        };
        match status {
            glow::FRAMEBUFFER_COMPLETE => Ok(()),
            glow::FRAMEBUFFER_UNDEFINED => Err("framebuffer undefined".to_owned()),
            glow::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => Err("framebuffer incomplete attachment".to_owned()),
            glow::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => Err("framebuffer incomplete missing attachment".to_owned()),
            glow::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => Err("framebuffer incomplete draw buffer".to_owned()),
            glow::FRAMEBUFFER_INCOMPLETE_READ_BUFFER => Err("framebuffer incomplete read buffer".to_owned()),
            glow::FRAMEBUFFER_UNSUPPORTED => Err("framebuffer unsupported".to_owned()),
            glow::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => Err("framebuffer incomplete multisample".to_owned()),
            glow::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => Err("framebuffer incomplete layer targets".to_owned()),
            _ => Err(format!("framebuffer error with status: {}", status)),
        }
    }

}

impl Drop for Framebuffer {

    fn drop(&mut self) {
        unsafe {
            self.gl.delete_framebuffer(self.id);
        }
    }

}

impl PartialEq for Framebuffer {

    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

}
