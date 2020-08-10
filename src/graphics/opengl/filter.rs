
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum FilterMode {
    Nearest,
    Linear,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Filter {
    pub min: FilterMode,
    pub mag: FilterMode,
    pub mipmap: Option<FilterMode>,
}

impl Filter {

    pub fn new(min: FilterMode, mag: FilterMode, mipmap: Option<FilterMode>) -> Self {
        Self { min, mag, mipmap }
    }

    pub(crate) fn to_min_flag(&self) -> u32 {
        match (self.min, self.mipmap) {
            (FilterMode::Nearest, None) => glow::NEAREST,
            (FilterMode::Linear, None) => glow::LINEAR,
            (FilterMode::Nearest, Some(FilterMode::Nearest)) => glow::NEAREST_MIPMAP_NEAREST,
            (FilterMode::Linear, Some(FilterMode::Nearest)) => glow::LINEAR_MIPMAP_NEAREST,
            (FilterMode::Nearest, Some(FilterMode::Linear)) => glow::NEAREST_MIPMAP_LINEAR,
            (FilterMode::Linear, Some(FilterMode::Linear)) => glow::LINEAR_MIPMAP_LINEAR,
        }
    }

    pub(crate) fn to_mag_flag(&self) -> u32 {
        match self.mag {
            FilterMode::Nearest => glow::NEAREST,
            FilterMode::Linear => glow::LINEAR,
        }
    }

}

impl Default for Filter {

    fn default() -> Self {
        Self::new(FilterMode::Nearest, FilterMode::Nearest, None)
    }

}
