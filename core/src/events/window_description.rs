use crate::entity::*;

pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Size { width, height }
    }
}

/// Passed to the window to set various window properties
pub struct WindowDescription {
    pub title: String,
    pub inner_size: Size,
    pub min_inner_size: Size,
    // Change this to resource id when the resource manager is working
    pub icon: Option<Vec<u8>>,
    pub icon_width: u32,
    pub icon_height: u32,
}

impl Default for WindowDescription {
    fn default() -> Self {
        Self {
            title: "Tuix Application".to_string(),
            inner_size: Size::new(800, 600),
            min_inner_size: Size::new(100, 100),
            icon: None,
            icon_width: 0,
            icon_height: 0,
        }
    }
}

impl WindowDescription {
    pub fn new() -> Self {
        WindowDescription {
            title: "Default".to_string(),
            inner_size: Size::new(800, 600),
            min_inner_size: Size::new(100, 100),
            icon: None,
            icon_width: 0,
            icon_height: 0,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();

        self
    }

    pub fn with_inner_size(mut self, width: u32, height: u32) -> Self {
        self.inner_size = Size::new(width, height);

        self
    }

    pub fn with_min_inner_size(mut self, width: u32, height: u32) -> Self {
        self.min_inner_size = Size::new(width, height);

        self
    }

    pub fn with_icon(mut self, icon: Vec<u8>, width: u32, height: u32) -> Self {
        self.icon = Some(icon);
        self.icon_width = width;
        self.icon_height = height;
        self
    }
}




pub struct WindowBuilder {
    entity: Entity,
    window_description: WindowDescription,
}

impl WindowBuilder {
    pub fn new(window: Entity) -> Self {
        Self {
            entity: window,
            window_description: WindowDescription::new(),
        }
    }

    pub fn get_window_description(&self) -> &WindowDescription {
        &self.window_description
    }

    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.window_description.title = title.to_string();

        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.window_description.title = title.to_string();

        self
    }

    pub fn set_inner_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.window_description.inner_size = Size::new(width, height);

        self
    }

    pub fn set_min_inner_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.window_description.min_inner_size = Size::new(width, height);

        self
    }

    pub fn set_icon(&mut self, icon: Vec<u8>, width: u32, height: u32) -> &mut Self {
        self.window_description.icon = Some(icon);
        self.window_description.icon_width = width;
        self.window_description.icon_height = height;
        self
    }


}

// impl AsEntity for WindowBuilder {
//     fn entity(&self) -> Entity {
//         self.entity
//     }
// }

// impl AsEntity for &WindowBuilder {
//     fn entity(&self) -> Entity {
//         self.entity
//     }
// }

// impl AsEntity for &mut WindowBuilder {
//     fn entity(&self) -> Entity {
//         self.entity
//     }
// }

// impl AsEntity for &&mut WindowBuilder {
//     fn entity(&self) -> Entity {
//         self.entity
//     }
// }

impl std::ops::Deref for WindowBuilder {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}