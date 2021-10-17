
//! # Tuix
//! 
//! Tuix is a cross-platform Graphical User Interface (GUI) framework based on simple ECS principles.
//!
//! In ECS terminology, UI widgets are identified by entities, style and layout properties are stored as components,
//! and a series of systems perform tasks such as layout, restyling, and drawing of the UI. In addition to these concepts,
//! there is also an event manager, which routes events between widgets, and a data binding system used for reactivity.
  

// mod accessibility;
// pub use accessibility::*;

pub mod animation;
pub use animation::*;

pub mod events;
pub use events::*;

mod id;
pub(crate) use id::IdManager;
pub use id::GenerationalId;

pub mod layout;
pub use layout::GeometryChanged;

pub mod state;
pub use state::*;

mod storage;

pub mod binding;
pub use binding::*;

pub mod style;
pub use style::*;

mod systems;
pub use crate::systems::*;

// mod text;
// pub use text::*;

pub mod tree;
pub use tree::*;

pub mod widget;
pub use widget::*;

pub mod window;
pub use window::*;


use femtovg::renderer::OpenGl;

pub use keyboard_types::{Code, Key};

pub type Canvas = femtovg::Canvas<OpenGl>;















