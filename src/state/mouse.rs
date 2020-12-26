use crate::entity::Entity;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MouseButtonState {
    Pressed,
    Released,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ModifiersState {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool,
}

impl Default for ModifiersState {
    fn default() -> Self {
        ModifiersState {
            shift: false,
            ctrl: false,
            alt: false,
            logo: false,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct MouseButtonData {
    pub state: MouseButtonState,
    pub pos_down: (f32, f32),
    pub pos_up: (f32, f32),
    pub pressed: Entity,
    pub released: Entity,
}

impl Default for MouseButtonData {
    fn default() -> Self {
        MouseButtonData {
            state: MouseButtonState::Released,
            pos_down: (0.0, 0.0),
            pos_up: (0.0, 0.0),
            pressed: Entity::null(),
            released: Entity::null(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct MouseState {
    pub cursorx: f32,
    pub cursory: f32,

    pub left: MouseButtonData,
    pub right: MouseButtonData,
    pub middle: MouseButtonData,
}

impl Default for MouseState {
    fn default() -> Self {
        MouseState {
            cursorx: 0.0,
            cursory: 0.0,
            left: MouseButtonData::default(),
            right: MouseButtonData::default(),
            middle: MouseButtonData::default(),
        }
    }
}
