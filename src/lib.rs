//! # Tuix
//! 
//! Tuix is a cross-platform Graphical User Interface (GUI) framework based on simple ECS principles.
//!
//! In ECS terminology, UI widgets are identified by entities, style and layout properties are stored as components,
//! and a series of systems perform tasks such as layout, restyling, and drawing of the UI. In addition to these concepts,
//! there is also an event manager, which routes events between widgets, and a data binding system used for reactivity.
  



pub use tuix_internal::*;

#[cfg(feature = "dynamic")]
#[allow(unused_imports)]
use tuix_dylib;