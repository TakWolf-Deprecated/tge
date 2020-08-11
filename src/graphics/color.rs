
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Color {
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const TRANSPARENT_WHITE: Self = Self::new(1.0, 1.0, 1.0, 0.0);
    pub const TRANSPARENT_BLACK: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.0);
    pub const YELLOW: Self = Self::new(1.0, 1.0, 0.0, 1.0);
    pub const CYAN: Self = Self::new(0.0, 1.0, 1.0, 1.0);
    pub const MAGENTA: Self = Self::new(1.0, 0.0, 1.0, 1.0);

    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self { red, green, blue, alpha }
    }

    pub fn from_u8(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let red = red as f32 / 255.0;
        let green = green as f32 / 255.0;
        let blue = blue as f32 / 255.0;
        let alpha = alpha as f32 / 255.0;
        Self::new(red, green, blue, alpha)
    }

    pub fn from_u32(color: u32) -> Self {
        let red = (color >> 24) as u8;
        let green = (color >> 16) as u8;
        let blue = (color >> 8) as u8;
        let alpha = color as u8;
        Self::from_u8(red, green, blue, alpha)
    }

    pub fn red_as_u8(&self) -> u8 {
        (self.red * 255.0).round() as u8
    }

    pub fn green_as_u8(&self) -> u8 {
        (self.green * 255.0).round() as u8
    }

    pub fn blue_as_u8(&self) -> u8 {
        (self.blue * 255.0).round() as u8
    }

    pub fn alpha_as_u8(&self) -> u8 {
        (self.alpha * 255.0).round() as u8
    }

    pub fn as_u32(&self) -> u32 {
        let red = (self.red_as_u8() as u32) << 24;
        let green = (self.green_as_u8() as u32) << 16;
        let blue = (self.blue_as_u8() as u32) << 8;
        let alpha = self.alpha_as_u8() as u32;
        red | green | blue | alpha
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from((red, green, blue, alpha): (f32, f32, f32, f32)) -> Self {
        Self::new(red, green, blue, alpha)
    }
}

impl Into<(f32, f32, f32, f32)> for Color {
    fn into(self) -> (f32, f32, f32, f32) {
        (self.red, self.green, self.blue, self.alpha)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((red, green, blue, alpha): (u8, u8, u8, u8)) -> Self {
        Self::from_u8(red, green, blue, alpha)
    }
}

impl Into<(u8, u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8, u8) {
        (self.red_as_u8(), self.green_as_u8(), self.blue_as_u8(), self.alpha_as_u8())
    }
}

impl From<u32> for Color {
    fn from(color: u32) -> Self {
        Self::from_u32(color)
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        self.as_u32()
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test_convert() {
        assert_eq!(Color::new(1.0, 0.0, 0.0, 0.0), Color::from_u8(255, 0, 0, 0));
        assert_eq!(Color::new(0.0, 1.0, 0.0, 0.0), Color::from_u8(0, 255, 0, 0));
        assert_eq!(Color::new(0.0, 0.0, 1.0, 0.0), Color::from_u8(0, 0, 255, 0));
        assert_eq!(Color::new(0.0, 0.0, 0.0, 1.0), Color::from_u8(0, 0, 0, 255));

        let color = Color::new(0.2, 0.4, 0.6, 0.8);
        assert_eq!(Color::from_u8(color.red_as_u8(), color.green_as_u8(), color.blue_as_u8(), color.alpha_as_u8()), color);
        assert_eq!(Color::from_u32(color.as_u32()), color);
    }
}
