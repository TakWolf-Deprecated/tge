use crate::math::{Vector, Size};

pub type LogicalPosition = Vector<f32>;
pub type PhysicalPosition = Vector<i32>;

impl LogicalPosition {
    pub fn from_physical(physical_position: impl Into<PhysicalPosition>, scale_factor: f32) -> Self {
        let physical_position = {
            let physical_position = physical_position.into();
            winit::dpi::PhysicalPosition::new(physical_position.x, physical_position.y)
        };
        let logical_position = winit::dpi::LogicalPosition::from_physical(physical_position, scale_factor as f64);
        Self::new(logical_position.x, logical_position.y)
    }

    pub fn to_physical(&self, scale_factor: f32) -> PhysicalPosition {
        let logical_position = winit::dpi::LogicalPosition::new(self.x, self.y);
        let physical_position = logical_position.to_physical(scale_factor as f64);
        PhysicalPosition::new(physical_position.x, physical_position.y)
    }
}

impl PhysicalPosition {
    pub fn from_logical(logical_position: impl Into<LogicalPosition>, scale_factor: f32) -> Self {
        let logical_position = {
            let logical_position = logical_position.into();
            winit::dpi::LogicalPosition::new(logical_position.x, logical_position.y)
        };
        let physical_position = winit::dpi::PhysicalPosition::from_logical(logical_position, scale_factor as f64);
        Self::new(physical_position.x, physical_position.y)
    }

    pub fn to_logical(&self, scale_factor: f32) -> LogicalPosition {
        let physical_position = winit::dpi::PhysicalPosition::new(self.x, self.y);
        let logical_position = physical_position.to_logical(scale_factor as f64);
        LogicalPosition::new(logical_position.x, logical_position.y)
    }
}

pub type LogicalSize = Size<f32>;
pub type PhysicalSize = Size<u32>;

impl LogicalSize {
    pub fn from_physical(physical_size: impl Into<PhysicalSize>, scale_factor: f32) -> Self {
        let physical_size = {
            let physical_size = physical_size.into();
            winit::dpi::PhysicalSize::new(physical_size.width, physical_size.height)
        };
        let logical_size = winit::dpi::LogicalSize::from_physical(physical_size, scale_factor as f64);
        Self::new(logical_size.width, logical_size.height)
    }

    pub fn to_physical(&self, scale_factor: f32) -> PhysicalSize {
        let logical_size = winit::dpi::LogicalSize::new(self.width, self.height);
        let physical_size = logical_size.to_physical(scale_factor as f64);
        PhysicalSize::new(physical_size.width, physical_size.height)
    }
}

impl PhysicalSize {
    pub fn from_logical(logical_size: impl Into<LogicalSize>, scale_factor: f32) -> Self {
        let logical_size = {
            let logical_size = logical_size.into();
            winit::dpi::LogicalSize::new(logical_size.width, logical_size.height)
        };
        let physical_size = winit::dpi::PhysicalSize::from_logical(logical_size, scale_factor as f64);
        Self::new(physical_size.width, physical_size.height)
    }

    pub fn to_logical(&self, scale_factor: f32) -> LogicalSize {
        let physical_size = winit::dpi::PhysicalSize::new(self.width, self.height);
        let logical_size = physical_size.to_logical(scale_factor as f64);
        LogicalSize::new(logical_size.width, logical_size.height)
    }
}
