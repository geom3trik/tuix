pub mod state;
pub use state::*;

pub mod application;
pub mod window;

pub use application::Application;
pub use window::{CursorIcon, KeyboardInput, Window, WindowEvent};

pub mod events;
pub use events::*;

pub use entity::{Entity, EntityManager};

pub mod widgets;
pub use crate::widgets::*;

pub mod systems;
pub use crate::systems::*;

pub use glutin::event::VirtualKeyCode;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
